use crate::brush::BrushId;

pub struct Canvas {
    current_brush: BrushId,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            current_brush: BrushId::DEFAULT,
        }
    }

    pub fn set_brush(&mut self, brush: BrushId) {
        self.current_brush = brush;
    }
}
