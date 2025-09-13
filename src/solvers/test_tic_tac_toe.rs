use abstract_game::{
  test_games::{TTTMove, TicTacToe},
  Game, ScoreValue, Solver,
};

use googletest::{gtest, prelude::*};
use rstest::rstest;
use rstest_reuse::{apply, template};

use crate::solvers::{alpha_beta::AlphaBeta, simple::SimpleSolver, ttable_solver::TTSolver};

/// Only includes solvers which find the true optimal moves (e.g.
/// highest-valued `Score`), which differs from optimal in terms of "never
/// loses" in that the minimum path to victory is required.
#[template]
#[rstest]
fn complete_solvers(
  #[values(SimpleSolver::new(), AlphaBeta::new(), TTSolver::new())] solver: (impl Solver),
) {
}

#[apply(complete_solvers)]
#[gtest]
fn test_solve(mut solver: impl Solver<Game = TicTacToe>) {
  let mut ttt = TicTacToe::new();
  {
    let (score, m) = solver.best_move(&ttt, 9);
    expect_eq!(score.score_at_depth(9), ScoreValue::Tie, "{score}");
    expect_that!(m, some(anything()));
  }

  // . . .
  // . . .
  // X . .
  ttt.make_move(TTTMove::new((0, 0)));
  {
    let (score, m) = solver.best_move(&ttt, 8);
    expect_eq!(score.score_at_depth(8), ScoreValue::Tie, "{score}");
    expect_that!(
      m,
      some(any![
        eq(TTTMove::new((0, 1))),
        eq(TTTMove::new((1, 1))),
        eq(TTTMove::new((1, 0))),
      ])
    );
  }

  // . . .
  // . . .
  // X . O
  ttt.make_move(TTTMove::new((2, 0)));
  {
    let (score, m) = solver.best_move(&ttt, 7);
    expect_eq!(
      score.score_at_depth(7),
      ScoreValue::CurrentPlayerWins,
      "{score}"
    );
    expect_that!(m, some(eq(TTTMove::new((2, 2)))));
  }

  // . . X
  // . . .
  // X . O
  ttt.make_move(TTTMove::new((2, 2)));
  {
    let (score, m) = solver.best_move(&ttt, 6);
    expect_eq!(
      score.score_at_depth(6),
      ScoreValue::OtherPlayerWins,
      "{score}"
    );
    expect_that!(m, some(eq(TTTMove::new((1, 1)))));
  }

  // . . X
  // . O .
  // X . O
  ttt.make_move(TTTMove::new((1, 1)));
  {
    let (score, m) = solver.best_move(&ttt, 5);
    expect_eq!(
      score.score_at_depth(5),
      ScoreValue::CurrentPlayerWins,
      "{score}"
    );
    expect_that!(m, some(eq(TTTMove::new((0, 2)))));
  }

  // X . X
  // . O .
  // X . O
  ttt.make_move(TTTMove::new((0, 2)));
  {
    let (score, m) = solver.best_move(&ttt, 5);
    expect_eq!(
      score.score_at_depth(5),
      ScoreValue::OtherPlayerWins,
      "{score}"
    );
    expect_that!(m, some(anything()));
  }

  // X . X
  // O O .
  // X . O
  ttt.make_move(TTTMove::new((0, 1)));
  {
    let (score, m) = solver.best_move(&ttt, 4);
    expect_eq!(
      score.score_at_depth(4),
      ScoreValue::CurrentPlayerWins,
      "{score}"
    );
    expect_that!(m, some(eq(TTTMove::new((1, 2)))));
  }
}
