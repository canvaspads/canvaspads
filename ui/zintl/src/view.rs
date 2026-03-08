use crate::element::{Element, ElementContext, IntoElement};
use crate::hook::HookId;

pub struct Context<R> {
    phantom: std::marker::PhantomData<R>,
}

impl<R> Context<R> {
    pub(crate) fn new() -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }

    pub fn get_dependencies(&self) -> Vec<HookId> {
        todo!()
    }
}

pub trait View {
    type Output;

    fn init(&mut self, _cx: &mut Context<Self::Output>) {}
    fn render(&self, cx: &mut Context<Self::Output>) -> impl IntoElement<Output = Self::Output>;
    fn deinit(&mut self, _cx: &mut Context<Self::Output>) {}
}

impl<R, T: View<Output = R> + Copy + 'static> IntoElement for T {
    type Output = R;

    fn into_element(&self, cx: &mut ElementContext<R>) -> Element<R> {
        let mut view_cx = cx.view_cx();
        let inner = self.render(&mut view_cx).into_element(cx);

        Element::Binded {
            inner: Box::new(inner),
            dependencies: view_cx.get_dependencies(),
            builder: Box::new(*self),
        }
    }
}
