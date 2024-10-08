use engine::constant::PIPS_PER_HALF_BOARD;
use crate::stage_theme::height::Height;
use crate::stage_theme::percent::Percent;

pub mod height;
pub mod percent;

#[derive(Copy, Clone)]
pub struct StageTheme {
    pub numbers: [char; 15],
    pub dices: [char; 6],
    pub board_border: char,
    pub space: char,
    pub pips_separator: char,
    pub white_checker: char,
    pub black_checker: char,
    pub possible_move: char,
    pub focused_pip: char,
    pub up: char,
    pub down: char,
    pub right: char,
    pub left: char,

    pub pip_size: u8,
    /* height of the board without borders */
    pub height: Height,
    pub bore_off_column_width: usize,
    /* cut off peaces if their height is greater than N % of board height */
    pub peaces_cut_off_height_percent: Percent
}

impl StageTheme {
    pub fn get_max_size(&self) -> (usize, usize) {
        let half_width: usize = self.get_half_width();

        let horizontal_border_length: usize = 1 + self.bore_off_column_width + 1 + half_width + 2 + half_width + 1 + self.bore_off_column_width + 1;
        let vertical_border_length: usize = 1 + *self.height + 1;

        (horizontal_border_length, vertical_border_length)
    }

    pub fn get_half_width(&self) -> usize {
        (self.pip_size * PIPS_PER_HALF_BOARD + (PIPS_PER_HALF_BOARD - 1)) as usize
    }

    pub fn get_max_checkers_to_show(&self) -> usize {
        let board_height: usize = *self.height;
        let cut_off_height_percent: usize = *self.peaces_cut_off_height_percent as usize;

        board_height * cut_off_height_percent / 100
    }
}
