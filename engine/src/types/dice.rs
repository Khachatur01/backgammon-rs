use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct Dice(u8);

impl Dice {
    pub fn new(value: u8) -> Self {
        if value == 0 || value > 6 {
            panic!(
                "Can't create DicePair. \
                Invalid value: {value}.\
                Must be in the range [1 - 6]"
            );
        }

        Self(value)
    }
}

impl Deref for Dice {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
