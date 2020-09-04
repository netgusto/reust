use std::any::Any;
use std::rc::Rc;

use crate::lib::frontend_static_text::{node, TextNode};
use crate::lib::reust::*;

use crate::component::counter::CounterComponent;

pub struct AppComponent {
    pub increment: i32,
}

impl Component<TextNode> for AppComponent {
    fn render<'a>(
        &self,
        _: Rc<dyn Any + 'a>,
        _set_state: Box<dyn Fn(Rc<dyn Any>)>,
    ) -> El<TextNode> {
        El::Node(node("Root").add_children(vec![
            El::Node(node("# Header A")),
            El::Component(Box::new(CounterComponent {
                initial_counter: 26,
                increment: self.increment,
            })),
            El::Node(
                node("----------------------------").add_children(vec![El::Component(Box::new(
                    CounterComponent {
                        initial_counter: -80,
                        increment: self.increment,
                    },
                ))]),
            ),
            El::Node(node("Ctrl-c to quit")),
        ]))
    }
}
