pub struct Context {}

pub trait View {
    type Output;

    fn init(&mut self, cx: &mut Context);
    fn render(&self, cx: &mut Context) -> Self::Output;
}
