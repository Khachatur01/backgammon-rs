use std::ops::Deref;

pub struct DicePair {
    first: u8,
    second: u8,
}

impl DicePair {
    pub fn new(first: u8, second: u8) -> Self {
        if first == 0 || first > 6 || second == 0 || second > 6 {
            panic!(
                "Can't create DicePair. \
                Invalid value: one - {first}, two - {second}. \
                Must be in the range [1 - 6]"
            );
        }

        Self {
            first, second
        }
    }

    pub fn first(&self) -> u8 {
        self.first
    }

    pub fn second(&self) -> u8 {
        self.second
    }
}