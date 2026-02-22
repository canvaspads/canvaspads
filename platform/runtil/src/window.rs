use crate::{
    actor::{MainActor, MainMarker},
    driver::{WindowImpl, WindowManagerImpl},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WindowId(usize);

#[derive(Clone, Debug)]
pub struct WindowManager {
    inner: WindowManagerImpl,
}

impl WindowManager {
    pub(crate) fn new(inner: WindowManagerImpl) -> Self {
        WindowManager { inner }
    }

    pub fn create_window(&self, marker: MainMarker) -> MainActor<Window> {
        let inner = self.inner.create_window_impl();
        MainActor::new(marker, Window::new(inner))
    }
}

#[derive(Clone, Debug)]
pub struct Window {
    inner: WindowImpl,
}

impl Window {
    pub(crate) fn new(inner: WindowImpl) -> Self {
        Window { inner }
    }

    pub fn show(&self) {
        self.inner.show();
    }
}
