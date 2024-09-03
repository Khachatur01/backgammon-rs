use crate::types::pip::Pip;

#[derive(Clone, Copy)]
pub enum CheckerMove {
    Step(Pip, Pip),
    BearOff(Pip)
}
