use abstract_game::{
  complete_solver::CompleteSolver,
  determined_score::DeterminedScore,
  test_games::{TTTMove, TicTacToe},
  Game, Solver,
};

use googletest::{gtest, prelude::*};
use rstest::rstest;
use rstest_reuse::{apply, template};

use crate::solvers::{alpha_beta::AlphaBeta, simple::SimpleSolver, ttable_solver::TTSolver};

#[template]
#[rstest]
fn complete_solvers(
  #[values(SimpleSolver::new(), AlphaBeta::new(), TTSolver::new())] solver: (impl CompleteSolver),
) {
}

#[apply(complete_solvers)]
#[gtest]
fn test_solve(mut solver: impl CompleteSolver + Solver<Game = TicTacToe>) {
  let mut ttt = TicTacToe::new();
  {
    let (score, m) = solver.best_move_determined(&ttt, 9);
    expect_eq!(score, DeterminedScore::guaranteed_tie(), "{score}");
    expect_that!(m, some(anything()));
  }

  // . . .
  // . . .
  // X . .
  ttt.make_move(TTTMove::new((0, 0)));
  {
    let (score, m) = solver.best_move_determined(&ttt, 8);
    expect_eq!(score, DeterminedScore::guaranteed_tie(), "{score}");
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
    let (score, m) = solver.best_move_determined(&ttt, 7);
    expect_eq!(score, DeterminedScore::win(5), "{score}");
    expect_that!(m, some(eq(TTTMove::new((2, 2)))));
  }

  // . . X
  // . . .
  // X . O
  ttt.make_move(TTTMove::new((2, 2)));
  {
    let (score, m) = solver.best_move_determined(&ttt, 6);
    expect_eq!(score, DeterminedScore::lose(4), "{score}");
    expect_that!(m, some(eq(TTTMove::new((1, 1)))));
  }

  // . . X
  // . O .
  // X . O
  ttt.make_move(TTTMove::new((1, 1)));
  {
    let (score, m) = solver.best_move_determined(&ttt, 5);
    expect_eq!(score, DeterminedScore::win(3), "{score}");
    expect_that!(m, some(eq(TTTMove::new((0, 2)))));
  }

  // X . X
  // . O .
  // X . O
  ttt.make_move(TTTMove::new((0, 2)));
  {
    let (score, m) = solver.best_move_determined(&ttt, 5);
    expect_eq!(score, DeterminedScore::lose(2), "{score}");
    expect_that!(m, some(anything()));
  }

  // X . X
  // O O .
  // X . O
  ttt.make_move(TTTMove::new((0, 1)));
  {
    let (score, m) = solver.best_move_determined(&ttt, 4);
    expect_eq!(score, DeterminedScore::win(1), "{score}");
    expect_that!(m, some(eq(TTTMove::new((1, 2)))));
  }
}
