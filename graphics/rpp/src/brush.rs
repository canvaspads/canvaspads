#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BrushId(u32);

impl BrushId {
    pub const DEFAULT: Self = BrushId(0);
}
