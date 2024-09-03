use crate::stage::side_switched::SideSwitched;
use crate::stage::win::Win;

pub enum MovesCommited {
    Win(Win),
    SideSwitched(SideSwitched)
}
