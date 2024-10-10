
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Side {
    White,
    Black
}

impl Side {
    pub fn opponent(&self) -> Self {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}
