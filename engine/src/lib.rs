pub mod constant;
pub mod board;
pub mod types;
pub mod stage;

use crate::board::Board;
use crate::stage::start::Start;

pub fn start_game() -> Start {
    Start::new(Board::default(), vec![])
}
