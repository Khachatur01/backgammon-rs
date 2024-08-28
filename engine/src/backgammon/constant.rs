pub mod player;
pub mod error;


pub const PIPS_SIZE: u8 = 24;
pub const CHECKER_PER_PLAYER: u8 = 15;
pub const BOARD_HEIGHT: u8 = CHECKER_PER_PLAYER * 2 + 1;


pub const NUMBERS: &[char] = &['⑴', '⑵', '⑶', '⑷', '⑸', '⑹', '⑺', '⑻', '⑼', '⑽', '⑾', '⑿', '⒀', '⒁', '⒂'];
pub const DICES: &[char] = &['⚀', '⚁', '⚂', '⚃', '⚄', '⚅'];
pub const BOARD_BORDER: char = '█';
pub const SPACE: char = ' ';
pub const PIPS_SEPARATOR: char = '|';
pub const WHITE_CHECKER: char = '⛂';
pub const BLACK_CHECKER: char = '⛀';
pub const POSSIBLE_MOVE: char = '🞙';
pub const UP: char = '⮝';
pub const DOWN: char = '⮟';
pub const RIGHT: char = '⮞';
