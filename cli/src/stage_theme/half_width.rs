use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct HalfWidth(usize);

impl HalfWidth {
    pub fn new(half_width: usize) -> Self {
        if half_width % 6 != 0 {
            panic!(
                "Can't create HalfWidth. \
                Invalid value: {half_width}. \
                Value must be divisible by 6"
            );
        }

        Self (half_width)
    }
}

impl Deref for HalfWidth {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
