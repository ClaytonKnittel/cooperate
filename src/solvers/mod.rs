pub mod alpha_beta;
pub mod simple;
pub mod ttable_alpha_beta;
pub mod ttable_solver;

#[cfg(test)]
mod fuzz_solvers;
#[cfg(test)]
mod fuzz_table_solvers;
#[cfg(test)]
mod test_connect_three;
#[cfg(test)]
mod test_tic_tac_toe;
