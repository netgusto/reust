use std::io::{Stdout, Write};
use std::ops::Sub;
use std::rc::Rc;
use std::sync::Mutex;
use std::time::Instant;
use std::{thread::sleep, time::Duration};

use termion::color;
use termion::cursor;
use termion::cursor::Goto;
use termion::event::{Event, Key, MouseEvent};
use termion::input::Events;
use termion::raw::RawTerminal;
use termion::AsyncReader;

use crate::engine::*;

#[derive(Default)]
pub struct Position {
    pub left: u16,
    pub top: u16,
}

#[derive(Default)]
pub struct Dimension {
    pub width: u16,
    pub height: u16,
}

#[derive(Default)]
pub struct Style {
    pub border: bool,
}

#[derive(Default)]
pub struct EventHandlers {
    pub on_click: Option<MouseClickHandler>,
}

pub type MouseClickHandler = Rc<Mutex<dyn Fn()>>;

pub struct TUINode {
    pub pos: Position,
    pub dim: Dimension,
    pub style: Style,
    pub text: Option<String>,
    pub disabled: bool,
    pub event_handlers: EventHandlers,
}

impl Default for TUINode {
    fn default() -> Self {
        TUINode {
            pos: Position { left: 1, top: 1 },
            dim: Dimension {
                width: 1,
                height: 1,
            },
            style: Style {
                ..Default::default()
            },
            disabled: false,
            text: None,
            event_handlers: EventHandlers {
                ..Default::default()
            },
        }
    }
}

impl TUINode {
    pub fn new(left: u16, top: u16) -> TUINode {
        TUINode {
            pos: Position { left, top },
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn disable(mut self, dis: bool) -> Self {
        self.disabled = dis;
        self
    }

    pub fn set_border(mut self, b: bool) -> Self {
        self.style.border = b;
        self
    }

    pub fn set_text(mut self, t: Option<String>) -> Self {
        self.text = t;
        self
    }

    pub fn set_dimension(mut self, width: u16, height: u16) -> Self {
        self.dim = Dimension { width, height };
        self
    }

    pub fn set_on_click(mut self, handler: Option<MouseClickHandler>) -> Self {
        self.event_handlers.on_click = handler;
        self
    }
}

pub fn draw_graph(stdout: &mut RawTerminal<Stdout>, el: &RenderedEl<TUINode>) {
    write!(stdout, "{}{}", termion::clear::All, cursor::Hide).unwrap();
    draw_rendered(stdout, el);
    stdout.flush().unwrap();
}

fn draw_rendered(stdout: &mut RawTerminal<Stdout>, e: &RenderedEl<TUINode>) {
    match e {
        RenderedEl::Node(rel) => draw_rendered_el(stdout, rel),
        RenderedEl::Container(c) => draw_rendered_container(stdout, c),
        RenderedEl::None => {}
    }
}

fn draw_rendered_el(stdout: &mut RawTerminal<Stdout>, rel: &RenderedNode<TUINode>) {
    let b = &rel.payload;
    let left = b.pos.left;
    let top = b.pos.top;

    if b.disabled {
        write!(stdout, "{}", color::Fg(color::Yellow)).unwrap();
    }

    let text = match &b.text {
        Some(t) => t.as_str(),
        None => "",
    };

    if b.style.border && b.dim.height >= 3 {
        let width = if b.dim.width >= 2 { b.dim.width - 2 } else { 0 } as usize;
        write!(
            stdout,
            "{}{}{}{}{}{}",
            Goto(left + 1, top),
            "▀".repeat(width),
            Goto(left + 1, top + b.dim.height - 1),
            "▄".repeat(width),
            Goto(
                left + (b.dim.width / 2) - (text.len() as u16 / 2),
                top + (b.dim.height / 2)
            ),
            if width == 0 {
                ""
            } else if text.len() > width {
                text.split_at(width).0
            } else {
                text
            }
        )
        .unwrap();
        for line in top..top + b.dim.height {
            write!(
                stdout,
                "{}█{}█",
                Goto(left, line),
                Goto(left + b.dim.width - 1, line),
            )
            .unwrap();
        }
    } else {
        write!(stdout, "{}{}", Goto(left, top), text).unwrap();
    }

    if b.disabled {
        write!(stdout, "{}", color::Fg(color::Reset)).unwrap();
    }

    for ch in &rel.children {
        draw_rendered(stdout, ch);
    }
}

fn draw_rendered_container(stdout: &mut RawTerminal<Stdout>, cont: &[RenderedEl<TUINode>]) {
    for ch in cont {
        draw_rendered(stdout, ch);
    }
}

pub fn process_events(
    events_it: &mut Events<AsyncReader>,
    app: &Option<RenderedEl<TUINode>>,
) -> bool // true: quit application
{
    loop {
        let event = events_it.next();
        match event {
            None => return false,
            Some(Ok(Event::Key(Key::Char(c)))) => {
                if let 'q' = c {
                    return true;
                }
            }
            Some(Ok(Event::Mouse(me))) => match app {
                None => {}
                Some(n) => {
                    if let MouseEvent::Release(left, top) = me {
                        track_mouse_clicked(n, left, top);
                    }
                }
            },
            _ => (),
        }
    }
}

fn track_mouse_clicked(el: &RenderedEl<TUINode>, left: u16, top: u16) {
    match el {
        RenderedEl::None => {}
        RenderedEl::Node(node) => {
            let tuinode = &node.payload;

            if tuinode.disabled {
                return;
            }

            if aabb_contains(
                tuinode.pos.left,
                tuinode.pos.top,
                tuinode.dim.width,
                tuinode.dim.height,
                left,
                top,
            ) {
                if let Some(c) = &node.payload.event_handlers.on_click {
                    {
                        let my_box_arc = c.clone();
                        let my_box = my_box_arc.lock().unwrap();
                        (*my_box)();
                        return;
                    }
                }
            }

            for c in &node.children {
                track_mouse_clicked(c, left, top)
            }
        }
        RenderedEl::Container(container) => {
            for c in container {
                track_mouse_clicked(c, left, top)
            }
        }
    };
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

pub struct VSync {
    last: Option<Instant>,
    every: Duration,
}

impl VSync {
    pub fn new(every: Duration) -> VSync {
        VSync { last: None, every }
    }

    pub fn wait(&mut self) {
        let to_wait = match self.last {
            None => self.every,
            Some(t) => match Instant::now().duration_since(t) {
                d if d < self.every => self.every.sub(d),
                _ => Duration::new(0, 0),
            },
        };

        if to_wait.as_nanos() > 0 {
            sleep(to_wait);
        }

        self.last = Some(Instant::now());
    }
}
