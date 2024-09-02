use std::ops::Deref;
use crate::backgammon::constant::PIPS_SIZE;

#[derive(Copy, Clone, Debug)]
pub struct FromPip(u8);

impl FromPip {
    pub fn new(value: u8) -> Self {
        if value >= PIPS_SIZE {
            panic!(
                "Can't create FromPip. \
                Invalid value: {value}. \
                Must be in the range [0 - {PIPS_SIZE})"
            );
        }

        Self(value)
    }
}

impl Deref for FromPip {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}