use std::any::Any;
use std::rc::Rc;

use crate::lib::frontend_static_text::{node, TextNode};
use crate::lib::reust::*;

use crate::component::counter::CounterComponent;

pub struct AppComponent {
    pub increment: i32,
}

use crate::component::counter::CounterComponentState;

impl<'a> Component<'a, TextNode> for AppComponent {
    fn render(&self, _: Rc<CounterComponentState>, _set_state: SetState) -> El<TextNode> {
        El::Node(node(String::from("Root")).add_children(vec![
            El::Node(node(String::from("# Header A"))),
            El::Component(Box::new(CounterComponent {
                initial_counter: 26,
                increment: self.increment,
            })),
            El::Node(
                node(String::from("----------------------------")).add_children(vec![
                    El::Component(Box::new(CounterComponent {
                        initial_counter: -80,
                        increment: self.increment,
                    })),
                ]),
            ),
            El::Node(node(String::from("Ctrl-c to quit"))),
        ]))
    }
}
