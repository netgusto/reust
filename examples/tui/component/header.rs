use reust::engine::*;
use reust::frontend::tui::*;

pub struct HeaderProps {
    pub pos: Position,
    pub text: String,
}

pub fn header(props: HeaderProps) -> El<TUINode> {
    El::Node(Node::new(
        TUINode::new(props.pos.left, props.pos.top).set_text(Some(format!("# {}", props.text))),
    ))
}
