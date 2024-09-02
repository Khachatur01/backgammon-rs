use std::ops::Deref;
use crate::backgammon::constant::PIPS_SIZE;

#[derive(Clone, Copy, Debug)]
pub struct ToPip(u8);

impl ToPip {
    pub fn new(value: u8) -> Self {
        if value >= PIPS_SIZE {
            panic!(
                "Can't create ToPip. \
                Invalid value: {value}. \
                Must be in the range [0 - {PIPS_SIZE})"
            );
        }

        Self(value)
    }
}

impl Deref for ToPip {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}