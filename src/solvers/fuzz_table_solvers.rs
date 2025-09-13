use std::{
  collections::HashMap,
  hash::{BuildHasher, Hash},
};

use abstract_game::{
  test_games::{ConnectN, Nim, TicTacToe},
  Game, Score, Solver,
};
use googletest::gtest;
use rstest::rstest;
use rstest_reuse::{apply, template};

use crate::solvers::{
  iter_deep::IterativeDeepening, simple::SimpleSolver, ttable_alpha_beta::TTAlphaBeta,
  ttable_solver::TTSolver,
};

trait HasTable<G, S> {
  fn table(&self) -> &HashMap<G, Score, S>;
}

impl<G: Game + Hash + Eq, S: BuildHasher + Clone> HasTable<G, S> for TTSolver<G, S> {
  fn table(&self) -> &HashMap<G, Score, S> {
    TTSolver::table(self)
  }
}

impl<G: Game + Hash + Eq, S: BuildHasher + Clone> HasTable<G, S> for TTAlphaBeta<G, S> {
  fn table(&self) -> &HashMap<G, Score, S> {
    TTAlphaBeta::table(self)
  }
}

impl<G: Game + Hash + Eq, S: BuildHasher + Clone> HasTable<G, S> for IterativeDeepening<G, S> {
  fn table(&self) -> &HashMap<G, Score, S> {
    IterativeDeepening::table(self)
  }
}

#[template]
#[rstest]
fn games(
  #[values((Nim::new(20), 20), (TicTacToe::new(), 9), (ConnectN::new(4, 3, 3), 12))]
    starting_state: (impl Game<Move: Ord>, u32),
) {
}

#[apply(games)]
#[gtest]
fn test_ground_truth_table_solver<G: Game<Move: Ord> + Hash + Eq>(starting_state: (G, u32)) {
  let (starting_state, depth) = starting_state;

  let mut solver = TTSolver::new();

  solver.best_move(&starting_state, depth);

  for (game, &score) in solver.table() {
    let (expected_score, _) = SimpleSolver::new().best_move(game, depth);
    println!("{score} vs {expected_score}:\n{game:?}\n");
    assert_eq!(score, expected_score);
  }
}

#[template]
#[rstest]
fn solvers(
  #[values(
    (TTSolver::new(), TTAlphaBeta::new()),
    (TTSolver::new(), IterativeDeepening::new(),
  ))]
  solvers: (impl Solver + HasTable, impl Solver + HasTable),
  #[values(
    (Nim::new(20), 20),
    (TicTacToe::new(), 9),
    (ConnectN::new(4, 3, 3), 12),
    (ConnectN::new(5, 4, 3), 20),
  )]
  starting_state: (impl Game<Move: Ord>, u32),
) {
}

#[apply(solvers)]
#[gtest]
fn test_solve<G: Game<Move: Ord> + Hash + Eq, S: BuildHasher + Clone>(
  solvers: (
    impl Solver<Game = G> + HasTable<G, S>,
    impl Solver<Game = G> + HasTable<G, S>,
  ),
  starting_state: (G, u32),
) {
  let (mut solver1, mut solver2) = solvers;
  let (starting_state, depth) = starting_state;

  solver1.best_move(&starting_state, depth);
  solver2.best_move(&starting_state, depth);

  for (game, score) in solver2.table() {
    let expected_score = *solver1.table().get(game).unwrap();
    if *score != expected_score {
      println!("{score} vs {expected_score}:\n{game:?}\n");
    }
    assert!(
      score.compatible(expected_score),
      "{score} vs {expected_score} for state\n{game:?}"
    );
  }
}
