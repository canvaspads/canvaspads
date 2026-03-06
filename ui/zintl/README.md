# zintl: Building GUI application in Rust.

```rust
use zintl::*;

struct LabelButton {
	text: String,
	click_hook: HookId,
}

...

impl IntoElement for LabelButton {
	type Output = RenderNode;

	fn into_element(&self, _cx: &mut ElementContext<RenderNode>) -> Element<RenderNode> {
		Element {
			node: RenderNode::LabelButton(self.text.clone()),
			dependencies: vec![click_hook],
		}
	}
}

...

struct TodoApp {
	tasks: Store<Vec<Task>>,
	task_name: Store<String>,
	add_button_click: Signal<ClickEvent>,
}

...

impl View for TodoApp {
	type Output = RenderNode;
	
	fn init(&mut self, cx: &mut Context) {
		self.tasks = cx.store(|| {
			vec![]
		});
		self.task_name = cx.store(|| { "".into() });
		self.add_button_signal = cx.signal();
		cx.subscribe(add_button_signal, |cx, e: ClickEvent| {
			match e {
				ClickEvent::Click {..} => {
					let task_name_str: &String = cx.get_store(todo_name).unwrap();
					let tasks: &mut Vec<Task> = cx.get_store_mut(tasks).unwrap();
					tasks.push(Task::new(*task_name_str));
					cx.post_future(async {
					});
				}
			}
		});
	}

	fn render(&self, cx: &mut Context) -> impl IntoElement<Output = RenderNode> {
		Stack::new([
			Input::new().store_value(task_name),
			LabelButton::new("Add todo").trigger_click(add_button_hook),
			Stack::new(
				cx.use_store(tasks, |tasks| {
					cx.render_iter(tasks)
				}),
			),
		])
	}
}

impl<R, T: View<Output = R> IntoElement for T {
	type Output = RenderNode;

	fn into_element(&self, cx: &mut ElementContext<RenderNode>) -> Element<RenderNode> {
        self.render(&mut cx.cx_view()).into_element(&mut cx.cx_child())
	}
}
```
