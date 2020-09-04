use std::any::Any;
use std::rc::Rc;

use crate::lib::frontend_static_text::{node, TextNode};
use crate::lib::reust::*;

pub struct LeafComponent {
    pub over_100: bool,
}

use crate::component::counter::CounterComponentState;

impl<'a> Component<'a, TextNode> for LeafComponent {
    fn render(&self, _: Rc<CounterComponentState>, _set_state: SetState) -> El<TextNode> {
        if self.over_100 {
            El::Node(node(String::from("It's OVER 9000! (jk 100)")))
        } else {
            El::Node(node(String::from("The leaf")))
        }
    }
}
