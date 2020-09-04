use crate::lib::frontend_tui::*;
use crate::lib::reust::*;

pub struct ProgressBarProps {
    pub pos: Position,
    pub percent: i32,
}

pub fn progress_bar(props: ProgressBarProps) -> El<TUINode> {
    El::Node(Node::new(
        TUINode::new(props.pos.left, props.pos.top)
            .set_text(Some(format!("{} %", props.percent)))
            .set_border(true)
            .set_dimension(if props.percent <= 0 { 0 } else { props.percent } as u16, 3),
    ))
}
