use crate::game::constant::{CHECKER_PER_PLAYER, PIPS_SIZE};

pub struct Checkers {
    pub on_board: [u8; PIPS_SIZE],
    pub bore_off_count: u8,
}

impl Default for Checkers {
    fn default() -> Self {
        let mut white_checkers: [u8; PIPS_SIZE] = [0; PIPS_SIZE];

        white_checkers[0] = CHECKER_PER_PLAYER;

        Checkers {
            on_board: white_checkers,
            bore_off_count: 0,
        }
    }
}
