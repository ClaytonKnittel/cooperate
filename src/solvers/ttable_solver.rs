use std::{
  collections::{hash_map::Entry, HashMap},
  hash::{BuildHasher, Hash, RandomState},
};

use abstract_game::{complete_solver::CompleteSolver, Game, GameResult, Score, Solver};

pub struct TTSolver<G, S> {
  table: HashMap<G, Score, S>,
}

impl<G: Game + Hash + Eq> TTSolver<G, RandomState> {
  pub fn new() -> Self {
    Self {
      table: HashMap::new(),
    }
  }
}

impl<G: Game + Hash + Eq, S: BuildHasher + Clone> TTSolver<G, S> {
  pub fn with_hasher(hasher: S) -> Self {
    Self {
      table: HashMap::with_hasher(hasher),
    }
  }

  pub fn table(&self) -> &HashMap<G, Score, S> {
    &self.table
  }

  fn backstepped_score_for_game(&mut self, game: &G, depth: u32) -> Score {
    match game.finished() {
      GameResult::Win(player) => {
        if player == game.current_player() {
          return Score::lose(1);
        } else {
          return Score::win(1);
        }
      }
      GameResult::Tie => return Score::guaranteed_tie(),
      GameResult::NotFinished => {}
    }

    if let Some(&score) = self.table.get(game) {
      if score.determined(depth) {
        return score.backstep();
      }
    }

    let score = self.solve_impl(game, depth);

    match self.table.entry(game.clone()) {
      Entry::Occupied(mut entry) => {
        let merged = entry.get().merge(score);
        *entry.get_mut() = merged;
        merged
      }
      Entry::Vacant(entry) => {
        entry.insert(score);
        score
      }
    }
    .backstep()
  }

  fn solve_impl(&mut self, game: &G, depth: u32) -> Score {
    debug_assert!(matches!(game.finished(), GameResult::NotFinished));
    if depth == 0 {
      return Score::NO_INFO;
    }

    game
      .each_move()
      .map(|m| game.with_move(m))
      .map(|next_game| self.backstepped_score_for_game(&next_game, depth - 1))
      .reduce(|acc, score| acc.accumulate(score))
      .unwrap_or(Score::lose(1))
  }
}

impl<G: Game + Hash + Eq, H: BuildHasher + Clone> Solver for TTSolver<G, H> {
  type Game = G;

  fn best_move(&mut self, game: &G, depth: u32) -> (Score, Option<G::Move>) {
    debug_assert!(matches!(game.finished(), GameResult::NotFinished));
    if depth == 0 {
      return (Score::NO_INFO, None);
    }

    game
      .each_move()
      .map(|m| {
        (
          self.backstepped_score_for_game(&game.with_move(m), depth - 1),
          Some(m),
        )
      })
      .max_by_key(|(score, _)| score.clone())
      // If you can't make a move, you lose.
      .unwrap_or((Score::lose(1), None))
  }
}

impl<G: Game + Hash + Eq, H: BuildHasher + Clone> CompleteSolver for TTSolver<G, H> {}

#[cfg(test)]
mod tests {
  use abstract_game::{
    test_games::{Nim, TicTacToe},
    ScoreValue, Solver,
  };

  use googletest::{gtest, prelude::*};

  use crate::solvers::ttable_solver::TTSolver;

  #[gtest]
  fn test_solve_nim() {
    for sticks in 1..=20 {
      let depth = sticks + 1;
      let expected_winner = sticks % 3 != 0;

      let mut solver = TTSolver::new();
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

  /// Tests that the TT solver fully determines every game state that it
  /// visits.
  #[gtest]
  fn test_fully_determined() {
    let mut solver = TTSolver::new();
    // populate the table
    solver.best_move(&TicTacToe::new(), 9);

    for (game, score) in solver.table {
      assert!(score.fully_determined(), "{score}:\n{game:?}");
    }
  }
}
