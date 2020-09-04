use std::rc::Rc;
use std::{thread::sleep, time::Duration};

use reust::frontend::text::*;
use reust::prelude::*;

// mod component;
// use component::counter::CounterComponent;

fn main() {
    let state = new_state_store();
    loop {
        draw_graph(render_app_to_graph(&app(), state.clone()));
        sleep(Duration::from_millis(500));
    }
}

fn app() -> El<TextNode> {
    El::Component(Box::new(App { increment: 8 }))
}

#[derive(Clone)]
pub struct AppState {
    pub value: i32,
}

pub struct App {
    pub increment: i32,
}

impl StateReceiver<AppState> for App {}
impl Component<TextNode> for App {
    fn initial_state(&self) -> Rc<dyn std::any::Any> {
        Rc::new(AppState { value: 0 })
    }

    fn render(&self, state: Rc<BoxedState>, set_state: Rc<SetState>) -> El<TextNode> {
        let state = self.must_receive_state(state);

        set_state(Rc::new(AppState {
            value: state.value + self.increment,
        }));

        El::Container(vec![
            El::Node(
                node("# Header A")
                    .add_child(El::Node(node(&format!("The counter is {}", state.value)))),
            ),
            El::Node(node("Ctrl-c to quit")),
        ])
    }
}
