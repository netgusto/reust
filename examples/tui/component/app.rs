use reust::engine::*;
use reust::frontend::tui::*;

use crate::component::header::*;
use crate::component::settings_controls::SettingsControls;

pub fn app() -> El<TUINode> {
    El::Container(vec![
        header(HeaderProps {
            pos: Position { left: 1, top: 1 },
            text: "Reactive TUI experiment with Rust".to_string(),
        }),
        El::Component(Box::new(SettingsControls { increment: 10 })),
    ])
}
