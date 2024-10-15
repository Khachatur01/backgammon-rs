use crate::types::dice::Dice;

#[derive(Copy, Clone)]
pub struct DicePair {
    first: Dice,
    second: Dice,
}

impl DicePair {
    pub fn new(first: Dice, second: Dice) -> Self {

        Self {
            first, second
        }
    }

    pub fn first(&self) -> u8 {
        *self.first
    }

    pub fn second(&self) -> u8 {
        *self.second
    }
}
