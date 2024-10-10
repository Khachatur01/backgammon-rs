use crate::constant::MAX_PIPS;
use std::ops::Deref;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pip(u8);

impl Pip {
    pub fn new(value: u8) -> Self {
        if value >= MAX_PIPS {
            panic!(
                "Can't create Pip. \
                Invalid value: {value}. \
                Must be in the range [0 - {MAX_PIPS})"
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
