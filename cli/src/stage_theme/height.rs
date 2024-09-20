use std::ops::Deref;

pub struct Height(usize);

impl Height {
    pub fn new(height: usize) -> Self {
        if height <= 7 {
            panic!(
                "Can't create Height. \
                Invalid value: {height\
                }. \
                Value must greater then 7"
            );
        }

        Self (height)
    }
}

impl Deref for Height {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
