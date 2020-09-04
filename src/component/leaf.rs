use std::any::Any;
use std::rc::Rc;

use crate::lib::frontend_static_text::{node, TextNode};
use crate::lib::reust::*;

pub struct LeafComponent {
    pub over_100: bool,
}

impl Component<TextNode> for LeafComponent {
    fn render<'a>(
        &self,
        _: Rc<dyn Any + 'a>,
        _set_state: Box<dyn Fn(Rc<dyn Any>)>,
    ) -> El<TextNode> {
        if self.over_100 {
            El::Node(node("It's OVER 9000! (jk 100)"))
        } else {
            El::Node(node("The leaf"))
        }
    }
}
