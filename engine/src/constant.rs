pub mod player;
pub mod error;
pub mod result;

pub const PIPS_SIZE: u8 = 24;
pub const CHECKER_PER_PLAYER: u8 = 15;
pub const BOARD_HEIGHT: u8 = CHECKER_PER_PLAYER * 2 + 1;
