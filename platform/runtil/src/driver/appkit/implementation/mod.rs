use std::{
    ffi::c_void,
    sync::{Arc, RwLock},
};

use crossbeam::queue::SegQueue;

use super::binding::*;
use crate::{actor::MainMarker, task::MainTask, window::Window};

pub enum AppkitEventPumpError {
    LockError,
}

pub type AppkitEventPumpResult<T> = Result<T, AppkitEventPumpError>;

pub struct QueueSource {
    queue: SegQueue<MainTask>,
}

impl QueueSource {
    fn new() -> Arc<Self> {
        Arc::new(QueueSource {
            queue: SegQueue::new(),
        })
    }
}

pub struct AppkitEventPump {
    source: Arc<QueueSource>,
}

extern "C" fn callback(s_ptr: *const c_void) {
    // SAFETY: ct is null checked
    let source = unsafe { Arc::from_raw(s_ptr as *mut QueueSource) };
    if let Some(task) = source.queue.pop() {
        (task.f)(MainMarker::new());
    }
}

impl AppkitEventPump {
    pub fn new() -> Self {
        let source = QueueSource::new();
        let ptr = Arc::into_raw(source.clone());
        // SAFETY:
        unsafe { runtilappkit_init(ptr as *const c_void, callback) };

        AppkitEventPump { source }
    }

    pub(crate) fn set_task_and_schedule(&self, task: MainTask) -> AppkitEventPumpResult<()> {
        {
            self.source.queue.push(task);
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
