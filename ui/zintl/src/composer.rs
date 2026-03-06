use crate::element::{ElementContext, IntoElement};
use crate::view::{Context, View};

pub struct Composer<R, V>
where
    V: View<Output = R>,
{
    root: V,
}

impl<R, V> Composer<R, V>
where
    V: View<Output = R>,
{
    pub fn new(root: V) -> Self {
        Composer { root }
    }

    pub fn render(&self) -> R {
        let mut context = Context::new();
        let a = self.root.render(&mut context);
        let mut context = ElementContext::new();
        a.into_element(&mut context).inner
    }
}
