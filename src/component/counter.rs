use std::any::Any;
use std::rc::Rc;

use crate::lib::frontend_static_text::{node, TextNode};
use crate::lib::reust::*;

use crate::component::leaf::LeafComponent;

pub struct CounterComponent {
    pub initial_counter: i32,
    pub increment: i32,
}

#[derive(Clone)]
struct CounterComponentState {
    num: i32,
}

impl StatefulComponent<CounterComponentState> for CounterComponent {}
impl Component<TextNode> for CounterComponent {
    fn initial_state(&self) -> Rc<dyn Any> {
        Rc::new(CounterComponentState {
            num: self.initial_counter,
        })
    }

    fn render(&self, state: Rc<dyn Any>, set_state: SetState) -> El<TextNode> {
        let mut new_state = self.state_from_any(state);
        new_state.num += self.increment;
        let counter = new_state.num;
        set_state(Rc::new(new_state));

        if counter % 10 == 0 {
            El::Node(node(&format!("{} IS %10!; skipping!", counter)))
        } else {
            El::Node(
                node(&format!("The counter is: {}", counter)).add_child(El::Node(
                    node("Sub element").add_child(El::Component(Box::new(LeafComponent {
                        over_100: counter > 100,
                    }))),
                )),
            )
        }
    }
}
