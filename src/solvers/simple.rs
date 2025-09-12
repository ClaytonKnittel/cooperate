use std::marker::PhantomData;

use abstract_game::{Game, GameResult, Score, Solver};

pub struct SimpleSolver<G>(PhantomData<G>);

impl<G: Game> SimpleSolver<G> {
  pub fn new() -> Self {
    Self(PhantomData)
  }

  fn score_for_game(game: &G, depth: u32) -> Score {
    match game.finished() {
      GameResult::Win(player) => {
        if player == game.current_player() {
          Score::lose(1)
        } else {
          Score::win(1)
        }
      }
      GameResult::Tie => Score::guaranteed_tie(),
      GameResult::NotFinished => Self::solve_impl(game, depth - 1).backstep(),
    }
  }

  fn solve_impl(game: &G, depth: u32) -> Score {
    debug_assert!(matches!(game.finished(), GameResult::NotFinished));
    if depth == 0 {
      return Score::NO_INFO;
    }

    game
      .each_move()
      .map(|m| game.with_move(m))
      .map(|next_game| Self::score_for_game(&next_game, depth))
      .max()
      // If you can't make a move, you lose.
      .unwrap_or(Score::lose(1))
  }
}

impl<G: Game> Solver for SimpleSolver<G> {
  type Game = G;

  fn best_move(&mut self, game: &G, depth: u32) -> (Score, Option<G::Move>) {
    debug_assert!(matches!(game.finished(), GameResult::NotFinished));
    if depth == 0 {
      return (Score::NO_INFO, None);
    }

    game
      .each_move()
      .map(|m| {
        let next_game = game.with_move(m);
        let score = Self::score_for_game(&next_game, depth);
        (score, Some(m))
      })
      .max_by_key(|(score, _)| score.clone())
      // If you can't make a move, you lose.
      .unwrap_or((Score::lose(1), None))
  }
}

#[cfg(test)]
mod tests {
  use abstract_game::{test_games::Nim, ScoreValue, Solver};

  use googletest::{gtest, prelude::*};

  use crate::solvers::simple::SimpleSolver;

  #[gtest]
  fn test_solve_nim() {
    for sticks in 1..=20 {
      let depth = sticks + 1;
      let expected_winner = sticks % 3 != 0;

      let mut solver = SimpleSolver::new();
      let (score, best_move) = solver.best_move(&Nim::new(sticks), sticks + 1);

      expect_eq!(
        score.score_at_depth(depth),
        if expected_winner {
          ScoreValue::CurrentPlayerWins
        } else {
          ScoreValue::OtherPlayerWins
        },
        "Game with {sticks} sticks"
      );
      if expected_winner {
        expect_that!(best_move, some(eq(sticks % 3)));
      } else {
        expect_that!(best_move, some(anything()));
      }
    }
  }
}
