use zintl::composer::Composer;
pub use zintl::element::{Element, ElementContext, IntoElement};
pub use zintl::view::{Context, View};

#[derive(Clone, Debug)]
pub enum RenderNode {
    Text(String),
}

pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: String) -> Self {
        Text { content }
    }
}

impl IntoElement for Text {
    type Output = RenderNode;

    fn into_element(&self, _cx: &mut ElementContext<Self::Output>) -> Element<Self::Output> {
        Element::Normal(RenderNode::Text(self.content.clone()))
    }
}

pub struct App<E>
where
    E: IntoElement<Output = RenderNode> + 'static,
{
    composer: Composer<RenderNode, E>,
}

impl<E> App<E>
where
    E: IntoElement<Output = RenderNode> + 'static,
{
    pub fn new(root: E) -> Self {
        App {
            composer: Composer::new(root),
        }
    }

    pub fn render(&self) -> RenderNode {
        // TODO
        self.composer.render().unwrap()
    }
}
