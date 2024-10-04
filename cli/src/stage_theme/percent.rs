use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct Percent(u8);

impl Percent {
    pub fn new(percent: u8) -> Self {
        if percent > 100 {
            panic!(
                "Can't create Percent. \
                Invalid value: {percent}. \
                Value must be between 0 and 100"
            );
        }

        Self (percent)
    }
}

impl Deref for Percent {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
