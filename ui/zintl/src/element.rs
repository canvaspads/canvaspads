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

pub enum Element<R> {
    Binded {
        inner: Box<Element<R>>,
        dependencies: Vec<HookId>,
        builder: Box<dyn IntoElement<Output = R>>,
    },
    BindedList {
        inner: Vec<Element<R>>,
        dependencies: Vec<HookId>,
        builder: Box<dyn IntoElement<Output = R>>,
    },
    Normal(R),
    List {
        inner: Vec<Element<R>>,
    },
}
