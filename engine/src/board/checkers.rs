use crate::constant::{CHECKER_PER_PLAYER, MAX_PIPS};

#[derive(Copy, Clone)]
pub struct Checkers {
    pub on_board: [u8; MAX_PIPS as usize],
    pub bore_off_count: u8,
}

impl Default for Checkers {
    fn default() -> Self {
        let mut checkers: [u8; MAX_PIPS as usize] = [0; MAX_PIPS as usize];

        let last_pip: &mut u8 = checkers
            .last_mut()
            .expect("Can't create checkers. Pips didn't initialize."); /* impossible case */

        *last_pip = CHECKER_PER_PLAYER;

        Self {
            on_board: checkers,
            bore_off_count: 0,
        }
    }
}
