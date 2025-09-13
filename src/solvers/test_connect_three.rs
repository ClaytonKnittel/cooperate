use abstract_game::{
  test_games::{ConnectMove, ConnectN},
  Game, ScoreValue, Solver,
};

use googletest::{gtest, prelude::*};
use rstest::rstest;
use rstest_reuse::{apply, template};

use crate::solvers::{
  alpha_beta::AlphaBeta, iter_deep::IterativeDeepening, simple::SimpleSolver,
  ttable_alpha_beta::TTAlphaBeta, ttable_solver::TTSolver,
};

#[template]
#[rstest]
fn solvers(
  #[values(
    SimpleSolver::new(),
    AlphaBeta::new(),
    TTSolver::new(),
    TTAlphaBeta::new(),
    IterativeDeepening::new()
  )]
  solver: (impl Solver),
) {
}

#[apply(solvers)]
#[gtest]
fn test_lose_in_corner(mut solver: impl Solver<Game = ConnectN>) {
  let mut conn = ConnectN::new(4, 3, 3);
  conn.make_move(ConnectMove { col: 0 });

  let (score, m) = solver.best_move(&conn, 12);
  expect_eq!(score.score_at_depth(12), ScoreValue::OtherPlayerWins);
  expect_that!(
    m,
    some(any![eq(ConnectMove { col: 1 }), eq(ConnectMove { col: 2 })])
  );
}
