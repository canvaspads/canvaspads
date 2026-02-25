use crate::render::RenderNode;
use zintl::view::View as ZintlView;

pub trait View {
    fn init(&mut self, cx: &mut Context);
    fn render(&self, cx: &mut Context) -> RenderNode;
    fn deinit(&mut self, cx: &mut Context);
}

impl<T: View> T for ZintlView<Output = RenderNode> {
    fn init(&mut self, cx: &mut Context) {
        self.init(cx);
    }
    fn render(&self, cx: &mut Context) -> Self::Output {
        self.render(cx)
    }

    fn deinit(&mut self, cx: &mut Context) {
        self.deinit(cx);
    }
}
