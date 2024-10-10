pub mod player;
pub mod error;
pub mod result;

pub const MAX_PIPS: u8 = 24;

/* board */
pub const TOP_RIGHT_BOARD_LEFT_PIP: u8 = (MAX_PIPS / 4) * 3;
pub const TOP_RIGHT_BOARD_RIGHT_PIP: u8 = MAX_PIPS - 1;

pub const TOP_LEFT_BOARD_LEFT_PIP: u8 = MAX_PIPS / 2;
pub const TOP_LEFT_BOARD_RIGHT_PIP: u8 = TOP_RIGHT_BOARD_LEFT_PIP - 1;

pub const BOTTOM_LEFT_BOARD_RIGHT_PIP: u8 = MAX_PIPS / 4;
pub const BOTTOM_LEFT_BOARD_LEFT_PIP: u8 = TOP_LEFT_BOARD_LEFT_PIP - 1;

pub const BOTTOM_RIGHT_BOARD_RIGHT_PIP: u8 = 0;
pub const BOTTOM_RIGHT_BOARD_LEFT_PIP: u8 = BOTTOM_LEFT_BOARD_RIGHT_PIP - 1;
/* board */


pub const PIPS_PER_HALF_BOARD: u8 = MAX_PIPS / 4;
pub const CHECKER_PER_PLAYER: u8 = 15;
pub const BOARD_HEIGHT: u8 = CHECKER_PER_PLAYER * 2 + 1;
