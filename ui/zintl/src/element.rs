use crate::hook::HookId;
use crate::view::Context;

pub struct ElementContext<R> {
    phantom: std::marker::PhantomData<R>,
}

impl<R> ElementContext<R> {
    pub fn new() -> Self {
        ElementContext {
            phantom: std::marker::PhantomData,
        }
    }

    pub fn view_cx(&mut self) -> Context<R> {
        todo!()
    }
}

pub trait IntoElement {
    type Output;

    fn into_element(&self, cx: &mut ElementContext<Self::Output>) -> Element<Self::Output>;
}

pub struct Element<R> {
    pub inner: R,
    pub dependencies: Vec<HookId>,
}
