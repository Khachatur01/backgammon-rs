use cursive::backends::crossterm::crossterm::event::PushKeyboardEnhancementFlags;
use crate::stage_theme::half_width::HalfWidth;
use crate::stage_theme::height::Height;
use crate::stage_theme::percent::Percent;

pub mod half_width;
pub mod height;
pub mod percent;

pub struct StageTheme {
    pub numbers: [char; 15],
    pub dices: [char; 6],
    pub board_border: char,
    pub space: char,
    pub pips_separator: char,
    pub white_checker: char,
    pub black_checker: char,
    pub possible_move: char,
    pub up: char,
    pub down: char,
    pub right: char,

    /* width of the board's half without borders */
    pub half_width: HalfWidth,
    /* height of the board without borders */
    pub height: Height,
    pub bore_off_width: usize,
    /* cut off peaces if their height is greater than N % of board height */
    pub peaces_cut_off_height_percent: Percent
}

impl StageTheme {
    pub fn get_max_size(&self) -> (usize, usize) {
        let horizontal_border_length: usize = 1 + self.bore_off_width + 1 + *self.half_width + 2 + *self.half_width + self.bore_off_width + 1;
        let vertical_border_length: usize = 1 + *self.height + 1;

        (horizontal_border_length, vertical_border_length)
    }
}
