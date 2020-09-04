use std::{thread::sleep, time::Duration};

mod lib;

// use lib::frontend_static_text::{draw, TextNode};
use lib::frontend_tui::{draw, TUINode};
use lib::reust::*;

mod component;

use std::io::{stdout, Write};

use termion::async_stdin;
use termion::cursor;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{Events, MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::AsyncReader;

fn main() {
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let stdin = async_stdin();
    let mut events_it = stdin.events();

    let mut state = StateStore::new();
    let mut current_app: Option<RenderedEl<TUINode>> = None;
    loop {
        write!(stdout, "{}{}", termion::clear::All, cursor::Hide).unwrap();

        if let true = process_events(&mut events_it, &current_app) {
            break;
        }

        let rendered = run_app(&app(), &mut state);
        draw(&mut stdout, &rendered);
        current_app = rendered;
        stdout.flush().unwrap();
        sleep(Duration::from_millis(16));
    }
}

fn app() -> El<TUINode> {
    use std::sync::Arc;
    use std::sync::Mutex;
    El::Node(
        Node::new(
            TUINode::new(10, 10)
                .set_text(Some("Hello, World!".to_string()))
                .set_height(3)
                .set_width(20)
                .set_border(true)
                .set_on_click(Some(Arc::new(Mutex::new(|| panic!("CLICKEssssss"))))),
        )
        .add_child(El::Component(Box::new(Counter {}))),
    )
}

use std::any::Any;
use std::rc::Rc;

struct Counter {}
struct CounterState {
    #[allow(dead_code)]
    counter: i32,
}

impl StatefulComponent<CounterState> for Counter {}
impl Component<TUINode> for Counter {
    fn initial_state(&self) -> Rc<dyn Any> {
        Rc::new(CounterState { counter: 0 })
    }

    #[allow(unused_variables)]
    fn render<'a>(
        &self,
        state: Rc<dyn Any + 'a>,
        set_state: &mut dyn FnMut(Rc<dyn Any + 'a>),
    ) -> El<TUINode> {
        use std::sync::Arc;
        use std::sync::Mutex;

        El::Node(Node::new(
            TUINode::new(30, 30)
                .set_text(Some("Le bouton".to_string()))
                .set_width(30)
                .set_height(30)
                .set_border(true)
                .set_on_click(Some(Arc::new(Mutex::new(|| {
                    set_state(Rc::new(CounterState { counter: 42 }));
                })))),
        ))
    }
}

fn process_events(events_it: &mut Events<AsyncReader>, app: &Option<RenderedEl<TUINode>>) -> bool {
    loop {
        let event = events_it.next();
        match event {
            None => return false,
            Some(Ok(Event::Key(Key::Char(c)))) => {
                if let 'q' = c {
                    return true;
                }
            }
            Some(Ok(Event::Mouse(me))) => {
                if let MouseEvent::Release(left, top) = me {
                    track_mouse_clicked(app, left, top);
                }
            }
            _ => (),
        }
    }
}

fn track_mouse_clicked(el: &Option<RenderedEl<TUINode>>, left: u16, top: u16) {
    let node = match el {
        None => return,
        Some(v) => v,
    };

    let tuinode = &node.payload;

    if tuinode.disabled {
        return;
    }

    if aabb_contains(
        tuinode.left,
        tuinode.top,
        tuinode.width,
        tuinode.height,
        left,
        top,
    ) {
        if let Some(c) = &node.payload.on_click {
            {
                let my_box_arc = c.clone();
                let mut my_box = my_box_arc.lock().unwrap();
                (*my_box)();
            }
        }
    }

    for c in &node.children {
        track_mouse_clicked(&c, left, top)
    }
}

fn aabb_contains(
    left: u16,
    top: u16,
    width: u16,
    height: u16,
    point_left: u16,
    point_top: u16,
) -> bool {
    left <= point_left
        && left + width >= point_left
        && top <= point_top
        && top + height >= point_top
}
