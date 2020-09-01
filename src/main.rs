use std::{thread::sleep, time::Duration};

mod lib;

use lib::frontend_static_text::{draw, Payload as TextNode};
use lib::reust::*;

mod component;
use component::app::AppComponent;

fn main() {
    let mut state = StateStore::new();
    loop {
        draw(run_app(&app(), &mut state));
        sleep(Duration::from_millis(1000));
    }
}

fn app() -> El<TextNode> {
    El::Component(Box::new(AppComponent { increment: 8 }))
}
