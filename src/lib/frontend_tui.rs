use std::io::{Stdout, Write};

use termion::color;
use termion::cursor::Goto;
use termion::raw::RawTerminal;

use std::sync::{Arc, Mutex};

use super::reust::RenderedEl;

pub type MouseClickHandler = Arc<Mutex<dyn FnMut()>>;

pub struct TUINode {
    pub text: Option<String>,
    pub top: u16,
    pub left: u16,
    pub width: u16,
    pub height: u16,
    pub border: bool,
    pub disabled: bool,
    pub on_click: Option<MouseClickHandler>,
}

impl Default for TUINode {
    fn default() -> Self {
        TUINode {
            left: 1,
            top: 1,
            width: 1,
            height: 1,
            text: None,
            disabled: false,
            border: false,
            on_click: None,
        }
    }
}

impl TUINode {
    pub fn new(left: u16, top: u16) -> TUINode {
        TUINode {
            left,
            top,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn disable(mut self, dis: bool) -> Self {
        self.disabled = dis;
        self
    }

    pub fn set_border(mut self, b: bool) -> Self {
        self.border = b;
        self
    }

    pub fn set_text(mut self, t: Option<String>) -> Self {
        self.text = t;
        self
    }

    pub fn set_width(mut self, w: u16) -> Self {
        self.width = w;
        self
    }

    pub fn set_height(mut self, h: u16) -> Self {
        self.height = h;
        self
    }

    pub fn set_on_click(mut self, handler: Option<MouseClickHandler>) -> Self {
        self.on_click = handler;
        self
    }
}

pub fn draw(stdout: &mut RawTerminal<Stdout>, el: &Option<RenderedEl<TUINode>>) {
    match el {
        None => {}
        Some(rel) => {
            let b = &rel.payload;
            let left = b.left;
            let top = b.top;

            if b.disabled {
                write!(stdout, "{}", color::Fg(color::Yellow)).unwrap();
            }

            let text = match &b.text {
                Some(t) => t.as_str(),
                None => "",
            };

            if b.border && b.height >= 3 {
                let width = if b.width >= 2 { b.width - 2 } else { 0 } as usize;
                write!(
                    stdout,
                    "{}{}{}{}{}{}",
                    Goto(left + 1, top),
                    "▀".repeat(width),
                    Goto(left + 1, top + b.height - 1),
                    "▄".repeat(width),
                    Goto(
                        left + (b.width / 2) - (text.len() as u16 / 2),
                        top + (b.height / 2)
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
                for line in top..top + b.height {
                    write!(
                        stdout,
                        "{}█{}█",
                        Goto(left, line),
                        Goto(left + b.width - 1, line),
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
                draw(stdout, ch);
            }
        }
    }
}
