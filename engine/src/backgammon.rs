use crate::backgammon;
use crate::backgammon::board::Board;
use crate::backgammon::stage::start::Start;
use rand::Rng;

pub mod constant;
pub mod board;
pub mod types;
pub mod stage;

pub type Result<E> = std::result::Result<(), E>;

pub fn start_game() -> Start {
    Start::new(Board::new(), vec![])
}
