use abstract_game::{
  test_games::{ConnectN, Nim, TicTacToe},
  test_util::deterministic_random_unfinished_state,
  Game, Solver,
};
use googletest::gtest;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rstest::rstest;
use rstest_reuse::{apply, template};

use crate::solvers::{
  alpha_beta::AlphaBeta, simple::SimpleSolver, ttable_alpha_beta::TTAlphaBeta,
  ttable_solver::TTSolver,
};

#[template]
#[rstest]
fn solvers(
  #[values(
    (SimpleSolver::new(), AlphaBeta::new()),
    (SimpleSolver::new(), TTSolver::new()),
    (SimpleSolver::new(), TTAlphaBeta::new())
  )]
  solvers: (impl Solver, impl Solver),
  #[values((Nim::new(20), 13), (TicTacToe::new(), 8), (ConnectN::new(4, 3, 3), 11))]
  starting_state: (impl Game<Move: Ord>, u32),
) {
}

#[apply(solvers)]
#[gtest]
fn test_solve<G: Game<Move: Ord>>(
  solvers: (impl Solver<Game = G>, impl Solver<Game = G>),
  starting_state: (G, u32),
) {
  let (mut solver1, mut solver2) = solvers;
  let (starting_state, expected_num_moves) = starting_state;

  let mut rng = StdRng::seed_from_u64(0x190214888295);

  const NUM_TRIALS: u32 = 500;
  for trial in 0..NUM_TRIALS {
    let n_moves = rng
      .random_range(0..=expected_num_moves)
      .max(rng.random_range(0..=expected_num_moves));
    let depth = rng
      .random_range(0..=expected_num_moves)
      .max(rng.random_range(0..=expected_num_moves));
    let mut game = starting_state.clone();
    deterministic_random_unfinished_state(&mut game, n_moves as usize, &mut rng).unwrap();

    let (score1, _) = solver1.best_move(&game, depth);
    let (score2, _) = solver2.best_move(&game, depth);

    assert!(score1.determined(depth), "Failed on trial {trial}");
    assert!(
      score2.determined(depth),
      "Failed on trial {trial}, {score2} not determined at depth {depth}"
    );
    assert!(
      score1.compatible(score2),
      "Failed on trial {trial}, {score1} vs {score2} for game\n{game:?}"
    );
  }
}
