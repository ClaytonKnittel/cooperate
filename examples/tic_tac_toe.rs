use abstract_game::{
  human_players::tic_tac_toe_player::TicTacToePlayer,
  interactive::{
    bot_player::BotPlayer, human_term_player::HumanTermPlayer, term_interface::TermInterface,
  },
  test_games::TicTacToe,
};
use cooperate::solvers::simple::SimpleSolver;

fn main() {
  let player1 = HumanTermPlayer::new("Player 1".to_owned(), TicTacToePlayer);
  let player2 = BotPlayer::new("Player 2".to_owned(), SimpleSolver::new(), 8);
  let game = TicTacToe::new();

  let result = TermInterface::new(game, player1, player2).map(TermInterface::play);
  if let Err(err) = result {
    println!("{err}");
  }
}
