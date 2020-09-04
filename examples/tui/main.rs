use std::io::stdout;
use std::time::Duration;

use reust::frontend::tui::*;
use reust::prelude::*;

mod component;
use component::app::app;

use termion::async_stdin;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let stdin = async_stdin();
    let mut events_it = stdin.events();

    let mut vsync = VSync::new(Duration::from_millis(16));

    let state = new_state_store();
    let mut current_graph = None;

    loop {
        if process_events(&mut events_it, &current_graph) {
            break;
        }

        let graph = render_app_to_graph(&app(), state.clone());
        draw_graph(&mut stdout, &graph);
        current_graph = Some(graph);

        vsync.wait();
    }
}
