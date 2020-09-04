use std::any::Any;
use std::rc::Rc;
use std::sync::Mutex;

use reust::engine::*;
use reust::frontend::tui::*;

use crate::component::button::*;
use crate::component::progress_bar::*;

pub struct SettingsControls {
    pub increment: i32,
}

#[derive(Clone)]
struct SettingsControlsState {
    percent: i32,
}

impl StateReceiver<SettingsControlsState> for SettingsControls {}
impl Component<TUINode> for SettingsControls {
    fn initial_state(&self) -> Rc<dyn Any> {
        Rc::new(SettingsControlsState { percent: 50 })
    }

    fn render(&self, state: Rc<BoxedState>, set_state: Rc<SetState>) -> El<TUINode> {
        let state = self.must_receive_state_rc(state);

        El::Container(vec![
            button(ButtonProps {
                pos: Position { left: 10, top: 10 },
                title: "Less".to_string(),
                disable: state.percent <= 0,
                on_click: Some(handle_on_less(
                    state.clone(),
                    set_state.clone(),
                    self.increment,
                )),
            }),
            button(ButtonProps {
                pos: Position { left: 45, top: 10 },
                title: "Moar!".to_string(),
                disable: state.percent >= 100,
                on_click: Some(handle_on_more(
                    state.clone(),
                    set_state.clone(),
                    self.increment,
                )),
            }),
            progress_bar(ProgressBarProps {
                pos: Position { left: 10, top: 20 },
                percent: state.percent,
            }),
            match state.percent {
                x if x <= 0 => El::Node(Node::new(
                    TUINode::new(50, 27).set_text(Some("Can't go lower than 0!".to_string())),
                )),
                x if x >= 100 => El::Node(Node::new(
                    TUINode::new(50, 27).set_text(Some("You're at the maximum!".to_string())),
                )),
                _ => El::None,
            },
        ])
    }
}

fn handle_on_less(
    state: Rc<SettingsControlsState>,
    set_state: Rc<SetState>,
    increment: i32,
) -> MouseClickHandler {
    Rc::new(Mutex::new(move || {
        set_state(Rc::new(SettingsControlsState {
            percent: if state.percent > increment {
                state.percent - increment
            } else {
                0
            },
        }));
    }))
}

fn handle_on_more(
    state: Rc<SettingsControlsState>,
    set_state: Rc<SetState>,
    increment: i32,
) -> MouseClickHandler {
    Rc::new(Mutex::new(move || {
        set_state(Rc::new(SettingsControlsState {
            percent: if state.percent + increment > 100 {
                100
            } else {
                state.percent + increment
            },
        }));
    }))
}
