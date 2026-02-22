use crate::window::WindowId;

pub enum WindowEvent {
    MouseDown,
}

pub enum Event {
    WindowEvent {
        window_id: WindowId,
        event: WindowEvent,
    },
}
