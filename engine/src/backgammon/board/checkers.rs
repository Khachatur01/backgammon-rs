use crate::backgammon::constant::{CHECKER_PER_PLAYER, PIPS_SIZE};

pub struct Checkers {
    pub on_board: [u8; PIPS_SIZE as usize],
    pub bore_off_count: u8,
}

impl Default for Checkers {
    fn default() -> Self {
        let mut white_checkers: [u8; PIPS_SIZE as usize] = [0; PIPS_SIZE as usize];

        if let Some(last_pip) = white_checkers.last_mut() {
            *last_pip = CHECKER_PER_PLAYER;
        }

        Self {
            on_board: white_checkers,
            bore_off_count: 0,
        }
    }
}
