use abstract_game::{
  complete_solver::CompleteSolver,
  test_games::{ConnectN, Nim, TicTacToe},
  test_util::deterministic_random_unfinished_state,
  Game, Solver,
};
use googletest::gtest;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rstest::rstest;
use rstest_reuse::{apply, template};

use crate::{
  solvers::{
    alpha_beta::AlphaBeta, iter_deep::IterativeDeepening, simple::SimpleSolver,
    ttable_alpha_beta::TTAlphaBeta, ttable_solver::TTSolver,
  },
  test::gomoku::Gomoku,
};

fn random_state<G: Game<Move: Ord>, R: Rng>(starting_state: &(G, u32, u32), rng: &mut R) -> G {
  let (starting_state, min_initial_moves, expected_num_moves) = starting_state;
  let min_initial_moves = *min_initial_moves;
  let expected_num_moves = *expected_num_moves;

  let n_moves = rng
    .random_range(min_initial_moves..=expected_num_moves)
    .max(rng.random_range(min_initial_moves..=expected_num_moves));
  let mut game = starting_state.clone();
  deterministic_random_unfinished_state(&mut game, n_moves as usize, rng).unwrap();
  game
}

#[template]
#[rstest]
fn solvers(
  #[values(
    (SimpleSolver::new(), AlphaBeta::new()),
    (SimpleSolver::new(), TTSolver::new()),
    (SimpleSolver::new(), TTAlphaBeta::new()),
    (SimpleSolver::new(), IterativeDeepening::new()),
  )]
  solvers: (impl Solver, impl Solver),
  #[values(
    (Nim::new(20), 0, 13),
    (TicTacToe::new(), 0, 8),
    (ConnectN::new(4, 3, 3), 0, 11),
    (Gomoku::new(4, 3, 3), 6, 6),
  )]
  starting_state: (impl Game<Move: Ord>, u32),
) {
}

#[apply(solvers)]
#[gtest]
fn test_solve<G: Game<Move: Ord>>(
  solvers: (impl Solver<Game = G>, impl Solver<Game = G>),
  starting_state: (G, u32, u32),
) {
  let (mut solver1, mut solver2) = solvers;
  let expected_num_moves = starting_state.2;

  let mut rng = StdRng::seed_from_u64(0x190214888295);

  const NUM_TRIALS: u32 = 500;
  for trial in 0..NUM_TRIALS {
    let game = random_state(&starting_state, &mut rng);
    let depth = rng
      .random_range(0..=expected_num_moves)
      .max(rng.random_range(0..=expected_num_moves));
    let (score1, _) = solver1.best_move(&game, depth);
    let (score2, _) = solver2.best_move(&game, depth);
    assert!(
      score1.compatible(score2),
      "Failed on trial {trial}, {score1} vs {score2} for game\n{game:?}"
    );
  }
}

#[template]
#[rstest]
fn determined_solvers(
  #[values(
    (SimpleSolver::new(), TTSolver::new()),
  )]
  solvers: (impl CompleteSolver, impl CompleteSolver),
  #[values(
    (Nim::new(20), 0, 13),
    (TicTacToe::new(), 0, 8),
    (ConnectN::new(4, 3, 3), 0, 11),
    (Gomoku::new(4, 3, 3), 6, 6),
  )]
  starting_state: (impl Game<Move: Ord>, u32),
) {
}

#[apply(determined_solvers)]
#[gtest]
fn test_solve_determined<G: Game<Move: Ord>>(
  solvers: (impl CompleteSolver<Game = G>, impl CompleteSolver<Game = G>),
  starting_state: (G, u32, u32),
) {
  let (mut solver1, mut solver2) = solvers;
  let expected_num_moves = starting_state.2;

  let mut rng = StdRng::seed_from_u64(0x438908592392);

  const NUM_TRIALS: u32 = 500;
  for trial in 0..NUM_TRIALS {
    let game = random_state(&starting_state, &mut rng);
    let depth = rng
      .random_range(1..=expected_num_moves)
      .max(rng.random_range(1..=expected_num_moves));
    let (score1, _) = solver1.best_move_determined(&game, depth);
    let (score2, _) = solver2.best_move_determined(&game, depth);

    let score1 = score1.truncated(depth);
    let score2 = score2.truncated(depth);

    assert_eq!(
      score1, score2,
      "Failed on trial {trial} to depth {depth}, {score1} vs {score2} for game\n{game:?}"
    );
  }
}
