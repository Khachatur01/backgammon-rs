use crate::types::pip::Pip;

pub enum Move {
    Step(Pip, Pip),
    BearOff(Pip)
}