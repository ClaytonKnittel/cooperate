use abstract_game::{
  human_players::connect_n_player::ConnectNPlayer,
  interactive::{
    bot_player::BotPlayer, human_term_player::HumanTermPlayer, term_interface::TermInterface,
  },
  test_games::ConnectN,
};
use cooperate::solvers::ttable_alpha_beta::TTAlphaBeta;

fn main() {
  let player1 = HumanTermPlayer::new("Player 1".to_owned(), ConnectNPlayer);
  let player2 = BotPlayer::new("Player 2".to_owned(), TTAlphaBeta::new(), 20);
  let game = ConnectN::new(5, 4, 3);

  let result = TermInterface::new(game, player1, player2).map(TermInterface::play);
  if let Err(err) = result {
    println!("{err}");
  }
}
