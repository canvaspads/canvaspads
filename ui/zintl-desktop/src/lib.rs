use zintl::composer::Composer;
pub use zintl::element::{Element, ElementContext, IntoElement};
pub use zintl::view::{Context, View};

#[derive(Clone, Debug)]
pub enum RenderNode {
    Text(String),
}

impl IntoElement for RenderNode {
    type Output = RenderNode;

    fn into_element(&self, _cx: &mut ElementContext<RenderNode>) -> Element<RenderNode> {
        Element::Normal {
            inner: (*self).clone(),
        }
    }
}

pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: String) -> Self {
        Text { content }
    }
}

impl View for Text {
    type Output = RenderNode;

    fn render(&self, _cx: &mut Context<RenderNode>) -> impl IntoElement<Output = RenderNode> {
        RenderNode::Text(self.content.clone())
    }
}

pub struct App<V>
where
    V: View<Output = RenderNode> + 'static,
{
    composer: Composer<RenderNode, V>,
}

impl<V> App<V>
where
    V: View<Output = RenderNode> + 'static,
{
    pub fn new(root: V) -> Self {
        App {
            composer: Composer::new(root),
        }
    }

    pub fn render(&self) -> RenderNode {
        // TODO
        self.composer.render().unwrap()
    }
}
