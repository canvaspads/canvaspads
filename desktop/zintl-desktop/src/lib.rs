pub mod components;
pub mod event;
pub mod hooks;
pub mod render;

use crate::render::RenderNode;
use zintl::sequence::Arena;
use zintl::view::View;

pub struct App<'a> {
    arena: Arena<'a>,
    root: Box<dyn View<Output = RenderNode>>,
}

impl<'a> App<'a> {
    pub fn new(root: impl View<Output = RenderNode> + 'static) -> Self {
        App {
            arena: Arena::new(),
            root: Box::new(root),
        }
    }

    pub fn start() {}
}
