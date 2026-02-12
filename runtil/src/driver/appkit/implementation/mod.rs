use std::{
    ffi::c_void,
    sync::{Arc, RwLock},
};

use super::binding::*;
use crate::{actor::MainMarker, task::MainTask, window::Window};

pub enum AppkitEventPumpError {
    LockError,
}

pub type AppkitEventPumpResult<T> = Result<T, AppkitEventPumpError>;

pub struct AppkitEventPump {
    ct: Arc<RwLock<Option<MainTask>>>,
}

extern "C" fn callback(ct: *const c_void) {
    // SAFETY: ct is null checked
    let optask = unsafe { Arc::from_raw(ct as *const RwLock<Option<MainTask>>) };
    if let Ok(mut optask) = optask.write() {
        if let Some(task) = (*optask).take() {
            (task.f)(MainMarker::new());
        }
    } //TODO: Error handling
}

impl AppkitEventPump {
    pub fn new() -> Self {
        let ct: Arc<RwLock<Option<MainTask>>> = Arc::new(None.into());
        let ptr = Arc::into_raw(ct.clone());
        // SAFETY:
        unsafe { runtilappkit_init(ptr as *const c_void, callback) };

        AppkitEventPump { ct }
    }

    pub(crate) fn set_task_and_schedule(&self, task: MainTask) -> AppkitEventPumpResult<()> {
        {
            let mut ct = match self.ct.write() {
                Ok(ct) => ct,
                Err(..) => return Err(AppkitEventPumpError::LockError),
            };

            *ct = Some(task);
        }

        // SAFETY+ TODO
        unsafe {
            runtilappkit_schedule();
        }

        Ok(())
    }

    pub fn create_window_manager_impl(&self) -> AppkitWindowManager {
        AppkitWindowManager::new()
    }

    pub fn run(&self) {
        unsafe { runtilappkit_run() };
    }

    pub fn quit(&self) {
        unsafe { runtilappkit_destroy() };
    }
}

#[derive(Clone, Debug)]
pub struct AppkitWindowManager {}

impl AppkitWindowManager {
    pub(crate) fn new() -> Self {
        AppkitWindowManager {}
    }

    pub(crate) fn create_window_impl(&self) -> AppkitWindow {
        AppkitWindow::new()
    }
}

#[derive(Clone, Debug)]
pub struct AppkitWindow {
    raw_ptr: *const c_void,
}

impl AppkitWindow {
    pub(crate) fn new() -> Self {
        let raw_ptr = unsafe { runtilappkit_create_window() };
        AppkitWindow { raw_ptr }
    }

    pub(crate) fn show(&self) {
        unsafe {
            runtilappkit_show_window(self.raw_ptr);
        };
    }
}
