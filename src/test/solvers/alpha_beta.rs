use std::ops::ControlFlow;

use abstract_game::{Game, GameResult, Score, Solver};

/// A temporary workaround until https://github.com/rust-lang/rust/issues/137461
/// is stabilized.
trait ControlFlowIntoValue<T> {
  fn custom_into_value(self) -> T;
}

impl<T> ControlFlowIntoValue<T> for ControlFlow<T, T> {
  fn custom_into_value(self) -> T {
    match self {
      ControlFlow::Continue(t) | ControlFlow::Break(t) => t,
    }
  }
}

pub struct AlphaBeta;

impl AlphaBeta {
  fn score_for_game<G: Game>(game: &G, depth: u32, alpha: Score, beta: Score) -> Score {
    match game.finished() {
      GameResult::Win(player) => {
        if player == game.current_player() {
          Score::lose(1)
        } else {
          Score::win(1)
        }
      }
      GameResult::Tie => Score::guaranteed_tie(),
      GameResult::NotFinished => {
        Self::solve_impl(game, depth - 1, beta.forwardstep(), alpha.forwardstep()).backstep()
      }
    }
  }

  fn solve_impl<G: Game>(game: &G, depth: u32, mut alpha: Score, beta: Score) -> Score {
    debug_assert!(matches!(game.finished(), GameResult::NotFinished));
    debug_assert!(alpha <= beta);
    if depth == 0 {
      return Score::no_info();
    }

    game
      .each_move()
      .map(|m| game.with_move(m))
      .map(|next_game| {
        let score = Self::score_for_game(&next_game, depth, alpha, beta);
        alpha = alpha.max(score);
        score
      })
      .try_fold(Score::lose(1), |best_score, score| {
        if score > beta {
          ControlFlow::Break(score)
        } else {
          ControlFlow::Continue(score.max(best_score))
        }
      })
      .custom_into_value()
  }
}

impl Solver for AlphaBeta {
  fn best_move<G: Game>(&mut self, game: &G, depth: u32) -> (Score, Option<G::Move>) {
    debug_assert!(matches!(game.finished(), GameResult::NotFinished));
    if depth == 0 {
      return (Score::no_info(), None);
    }

    let mut alpha = Score::lose(1);

    game
      .each_move()
      .map(|m| {
        let next_game = game.with_move(m);
        let score = Self::score_for_game(&next_game, depth, alpha, Score::win(1));
        alpha = alpha.max(score);
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

  use crate::test::solvers::alpha_beta::AlphaBeta;

  #[gtest]
  fn test_solve_nim() {
    for sticks in 1..=20 {
      let depth = sticks + 1;
      let expected_winner = sticks % 3 != 0;

      let mut solver = AlphaBeta;
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
