pub struct Context {}

pub trait View {
    type Output;

    fn init(&mut self, _cx: &mut Context) {}
    fn render(&self, cx: &mut Context) -> Self::Output;
    fn deinit(&mut self, _cx: &mut Context) {}
}
