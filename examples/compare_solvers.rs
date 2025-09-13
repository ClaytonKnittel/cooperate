use std::{
  hash::Hash,
  hint::black_box,
  time::{Duration, SystemTime},
};

use abstract_game::{test_games::ConnectN, Game, Solver};
use cooperate::solvers::{
  alpha_beta::AlphaBeta, iter_deep::IterativeDeepening, simple::SimpleSolver,
  ttable_alpha_beta::TTAlphaBeta, ttable_solver::TTSolver,
};

fn time_solver<G: Game>(
  mut solver: impl Solver<Game = G>,
  initial_state: &G,
  depth: u32,
) -> Duration {
  let start = SystemTime::now();
  let result = solver.best_move(initial_state, depth);
  black_box(result);

  SystemTime::now().duration_since(start).unwrap()
}

fn time_solvers<G: Game + Hash + Eq>(initial_state: &G, depth: u32) {
  println!(
    "Simple time: {:?}",
    time_solver(SimpleSolver::new(), initial_state, depth)
  );
  println!(
    "Alpha/beta time: {:?}",
    time_solver(AlphaBeta::new(), initial_state, depth)
  );
  println!(
    "Transposition table time: {:?}",
    time_solver(TTSolver::new(), initial_state, depth)
  );
  println!(
    "TT+AB time: {:?}",
    time_solver(TTAlphaBeta::new(), initial_state, depth)
  );
  println!(
    "Iter deep time: {:?}",
    time_solver(IterativeDeepening::new(), initial_state, depth)
  );
}

fn main() {
  time_solvers(&ConnectN::new(5, 4, 3), 13);
}
