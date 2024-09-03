use crate::types::pip::Pip;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CheckerMove {
    Play(Pip, Pip),
    BearOff(Pip)
}
