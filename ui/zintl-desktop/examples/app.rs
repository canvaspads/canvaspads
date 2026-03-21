use zintl_desktop::*;

#[derive(Copy, Clone)]
pub struct MainView {}

impl View for MainView {
    type Output = RenderNode;

    fn render(&self, _cx: &mut Context<RenderNode>) -> impl IntoElement<Output = RenderNode> {
        Text::new("hello, world!".to_string())
    }
}

fn main() {
    let app = App::new(MainView {});
    let node = app.render();
    println!("{:?}", node);
}
