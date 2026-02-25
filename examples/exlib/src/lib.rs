pub mod components;
pub mod event;
pub mod hooks;
pub mod render;
pub mod view;

use crate::view::View;
use zintl::sequence::Arena;

pub struct ExApp<'a> {
    arena: Arena<'a>,
    root: Box<dyn View>,
}

impl<'a> ExApp<'a> {
    pub fn new(root: impl View) -> Self {
        ExApp {
            arena: Arena::new(),
            root: Box::new(root),
        }
    }

    pub fn start() {}
}
