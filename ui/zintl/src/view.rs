use crate::element::{Element, ElementContext, IntoElement};

pub struct Context<R> {
    phantom: std::marker::PhantomData<R>,
}

impl<R> Context<R> {
    pub(crate) fn new() -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }

    pub fn child(&mut self) -> Self {
        Self::new()
    }
}

pub trait View {
    type Output;

    fn init(&mut self, _cx: &mut Context<Self::Output>) {}
    fn render(&self, cx: &mut Context<Self::Output>) -> impl IntoElement<Output = Self::Output>;
    fn deinit(&mut self, _cx: &mut Context<Self::Output>) {}
}

impl<R, T: View<Output = R>> IntoElement for T {
    type Output = R;

    fn into_element(&self, cx: &mut ElementContext<R>) -> Element<R> {
        self.render(&mut cx.view_cx()).into_element(cx)
    }
}
