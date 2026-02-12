use crate::{
    driver::{EventPumpImpl, WindowManagerImpl},
    task::MainTask,
};

pub(crate) struct MainThreadRunner {
    pump: EventPumpImpl,
}

impl MainThreadRunner {
    pub fn new() -> Self {
        MainThreadRunner {
            pump: EventPumpImpl::new(),
        }
    }

    pub fn schedule_task(&self, task: MainTask) {
        self.pump.set_task_and_schedule(task);
    }

    pub fn create_window_manager_impl(&self) -> WindowManagerImpl {
        self.pump.create_window_manager_impl()
    }

    pub fn run(&self) {
        self.pump.run();
    }

    pub fn quit(&self) {
        self.pump.quit();
    }
}
