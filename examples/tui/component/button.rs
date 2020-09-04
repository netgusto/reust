use reust::engine::*;
use reust::frontend::tui::*;

pub struct ButtonProps {
    pub pos: Position,
    pub title: String,
    pub on_click: Option<MouseClickHandler>,
    pub disable: bool,
}

impl Default for ButtonProps {
    fn default() -> Self {
        ButtonProps {
            pos: Position {
                ..Default::default()
            },
            title: "".to_string(),
            disable: false,
            on_click: None,
        }
    }
}

pub fn button(props: ButtonProps) -> El<TUINode> {
    El::Node(Node::new(
        TUINode::new(props.pos.left, props.pos.top)
            .set_text(Some(props.title.to_string()))
            .set_border(true)
            .set_dimension(12 + props.title.len() as u16, 5)
            .disable(props.disable)
            .set_on_click(props.on_click),
    ))
}
