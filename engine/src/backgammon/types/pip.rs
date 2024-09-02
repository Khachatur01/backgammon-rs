use std::ops::Deref;
use crate::backgammon::constant::PIPS_SIZE;
use crate::types::from_pip::FromPip;
use crate::types::to_pip::ToPip;

#[derive(Copy, Clone, Debug)]
pub struct Pip(u8);

impl From<FromPip> for Pip {
    fn from(value: FromPip) -> Self {
        Pip(*value)
    }
}
impl From<ToPip> for Pip {
    fn from(value: ToPip) -> Self {
        Pip(*value)
    }
}

impl Deref for Pip {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}