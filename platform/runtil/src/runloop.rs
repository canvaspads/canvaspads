use std::{
    marker::PhantomData,
    sync::{Arc, OnceLock},
    thread::{self, ThreadId},
};

use crate::{
    actor::MainMarker, event::Event, runner::MainThreadRunner, task::MainTask,
    window::WindowManager,
};

pub(crate) static MAIN_THREAD_ID: OnceLock<ThreadId> = OnceLock::new();

pub struct Context<M: UserMessage> {
    main_runner: Arc<MainThreadRunner>,
    phantom: PhantomData<M>,
}

impl<M: UserMessage> Context<M> {
    pub(crate) fn new(main_runner: Arc<MainThreadRunner>) -> Self {
        Context {
            main_runner,
            phantom: PhantomData,
        }
    }

    pub fn post_random_task(&self, _fut: impl Future<Output = ()>, _send_after: Option<M>) {
        unimplemented!();
    }

    pub fn dispatch_main(&self, f: impl Fn(MainMarker) -> () + 'static) {
        let task = MainTask { f: Box::new(f) };
        self.main_runner.schedule_task(task);
    }

    pub fn window_manager(&self) -> WindowManager {
        let inner = self.main_runner.create_window_manager_impl();
        WindowManager::new(inner)
    }
}

pub trait UserMessage {}

pub trait RunLoopHandler<M: UserMessage>
where
    Self: Send + Sync,
{
    fn init(&mut self, _cx: &mut Context<M>) {}
    fn handle_message(&mut self, _cx: &Context<M>, _msg: M) {}
    fn handle_event(&mut self, _cx: &Context<M>, _e: Event) {}
    fn quit(&mut self, _cx: &Context<M>) {}
}

pub struct RunLoop<M, H>
where
    M: UserMessage,
    H: RunLoopHandler<M>,
{
    main_runner: Arc<MainThreadRunner>,
    // worker_runner: ParallelRunner,
    handler: H,
    phantom: std::marker::PhantomData<M>,
    initialized: bool,
}

impl<M, H> RunLoop<M, H>
where
    M: UserMessage,
    H: RunLoopHandler<M>,
{
    pub fn new(handler: H) -> Self {
        let thread_id = thread::current().id();
        let _ = MAIN_THREAD_ID.set(thread_id);

        RunLoop {
            main_runner: Arc::new(MainThreadRunner::new()),
            handler,
            phantom: std::marker::PhantomData,
            initialized: true,
        }
    }

    fn create_context(&self) -> Context<M> {
        Context::new(self.main_runner.clone())
    }

    pub fn run(&mut self) {
        let mut context = self.create_context();
        self.handler.init(&mut context);
        self.main_runner.run();
    }

    pub fn quit(&mut self) {
        self.main_runner.quit();
        self.initialized = false;
    }
}

impl<M, H> Drop for RunLoop<M, H>
where
    M: UserMessage,
    H: RunLoopHandler<M>,
{
    fn drop(&mut self) {
        if self.initialized {
            self.quit();
        }
    }
}
