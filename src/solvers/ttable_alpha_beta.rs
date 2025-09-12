use std::{
  collections::{hash_map::Entry, HashMap},
  hash::{BuildHasher, Hash, RandomState},
};

use abstract_game::{Game, GameResult, Score, Solver};

pub struct TTAlphaBeta<G, S> {
  table: HashMap<G, Score, S>,
}

impl<G> TTAlphaBeta<G, RandomState> {
  pub fn new() -> Self {
    Self {
      table: HashMap::new(),
    }
  }
}

impl<G: Game + Hash + Eq, S: BuildHasher + Clone> TTAlphaBeta<G, S> {
  pub fn with_hasher(hasher: S) -> Self {
    Self {
      table: HashMap::with_hasher(hasher),
    }
  }

  pub fn table(&self) -> &HashMap<G, Score, S> {
    &self.table
  }

  fn backstepped_score_for_game(
    &mut self,
    game: &G,
    depth: u32,
    alpha: Score,
    beta: Score,
  ) -> Score {
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

    let score = self.solve_impl(game, depth, beta.forwardstep(), alpha.forwardstep());

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

  fn solve_impl(&mut self, game: &G, depth: u32, alpha: Score, beta: Score) -> Score {
    debug_assert!(matches!(game.finished(), GameResult::NotFinished));
    debug_assert!(alpha <= beta, "{alpha} vs {beta}");
    if depth == 0 {
      return Score::NO_INFO;
    }

    let mut best_score = Score::lose(1);
    for next_game in game.each_move().map(|m| game.with_move(m)) {
      let score =
        self.backstepped_score_for_game(&next_game, depth - 1, alpha.max(best_score), beta);
      best_score = best_score.max(score);
      if (score.determined(depth) && score.score_at_depth(depth).is_winning()) || score > beta {
        return best_score.break_early();
      }
    }

    best_score
  }
}

impl<G: Game + Hash + Eq, S: BuildHasher + Clone> Solver for TTAlphaBeta<G, S> {
  type Game = G;

  fn best_move(&mut self, game: &G, depth: u32) -> (Score, Option<G::Move>) {
    debug_assert!(matches!(game.finished(), GameResult::NotFinished));
    if depth == 0 {
      return (Score::NO_INFO, None);
    }

    let mut alpha = Score::lose(1);

    game
      .each_move()
      .map(|m| {
        let next_game = game.with_move(m);
        let score = self.backstepped_score_for_game(&next_game, depth - 1, alpha, Score::win(1));
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

  use super::TTAlphaBeta;

  #[gtest]
  fn test_solve_nim() {
    for sticks in 1..=20 {
      let depth = sticks + 1;
      let expected_winner = sticks % 3 != 0;

      let mut solver = TTAlphaBeta::new();
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
