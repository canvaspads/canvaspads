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

    pub fn render(&self) -> Option<R> {
        let mut context = Context::new();
        let a = self.root.render(&mut context);
        match a {
            _ => {}
        };
        None
    }
}
