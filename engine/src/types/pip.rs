use crate::constant::PIPS_SIZE;
use std::ops::Deref;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pip(u8);

impl Pip {
    pub fn new(value: u8) -> Self {
        if value >= PIPS_SIZE {
            panic!(
                "Can't create Pip. \
                Invalid value: {value}. \
                Must be in the range [0 - {PIPS_SIZE})"
            );
        }

        Self(value)
    }
}

impl Deref for Pip {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
