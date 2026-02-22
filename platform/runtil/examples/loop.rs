use runtil::{Context, RunLoop, RunLoopHandler, UserMessage};

struct Message();
impl UserMessage for Message {}
struct Handler();
impl RunLoopHandler<Message> for Handler {
    fn init(&mut self, cx: &mut Context<Message>) {
        let wm = cx.window_manager();
        cx.dispatch_main(move |marker| {
            let window = wm.create_window(marker);
            window.read(marker).unwrap().show();
            println!("hello, world!");
        });
    }
}

fn main() {
    let mut runloop = RunLoop::new(Handler());
    runloop.run();
}
