use std::rc::Rc;

use crate::lib::frontend_tui::*;
use crate::lib::reust::*;

use crate::component::header::*;
use crate::component::settings_controls::SettingsControls;

pub struct App {}

impl Component<TUINode> for App {
    fn render(&self, _state: Rc<BoxedState>, _set_state: Rc<SetState>) -> El<TUINode> {
        El::Container(vec![
            header(HeaderProps {
                pos: Position { left: 1, top: 1 },
                text: "Reactive TUI experiment with Rust".to_string(),
            }),
            El::Component(Box::new(SettingsControls { increment: 10 })),
        ])
    }
}
