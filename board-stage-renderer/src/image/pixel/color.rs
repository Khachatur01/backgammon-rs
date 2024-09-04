#[derive(Copy, Clone)]
pub struct Color(u8, u8, u8);

impl Default for Color {
    fn default() -> Self {
        Self(0, 0, 0)
    }
}
