use std::io::stdout;
use std::time::Duration;

mod lib;

use lib::frontend_tui::{draw, process_events, TUINode, VSync};
use lib::reust::*;

mod component;

use component::app::App;

use termion::async_stdin;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let stdin = async_stdin();
    let mut events_it = stdin.events();

    let state = new_state_store();
    let mut current_graph = None;

    let mut vsync = VSync::new(Duration::from_millis(16));

    loop {
        if let true = process_events(&mut events_it, &current_graph) {
            break;
        }

        let rendered = render_app(&app(), state.clone());
        draw(&mut stdout, &rendered);
        current_graph = Some(rendered);

        vsync.wait();
    }
}

fn app() -> El<TUINode> {
    El::Component(Box::new(App {}))
}
