pub mod constant;
pub mod board;
pub mod types;
pub mod stage;

use crate::board::Board;
use crate::stage::started::Started;

pub fn start_game() -> Started {
    Started::new(Board::default(), vec![])
}
