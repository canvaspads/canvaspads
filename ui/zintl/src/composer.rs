use crate::element::{Element, ElementContext, IntoElement};
use crate::view::{Context, View};

pub struct Composer<R, E>
where
    E: IntoElement<Output = R>,
{
    root: E,
}

impl<R, E> Composer<R, E>
where
    E: IntoElement<Output = R>,
{
    pub fn new(root: E) -> Self {
        Composer { root }
    }

    pub fn render(&self) -> Option<R> {
        let mut context = ElementContext::new();
        let a = self.root.into_element(&mut context);
        match a {
            Element::Normal(r) => Some(r),
            _ => None,
        }
    }
}
