use super::reust::{El, Node, RenderedEl};

#[derive(Clone)]
pub struct Payload {
    pub text: String,
}

impl Payload {
    pub fn new(text: &str) -> Self {
        Self {
            text: String::from(text),
        }
    }
}

pub fn node(text: &str) -> Node<Payload, El<Payload>> {
    Node::new(Payload::new(text))
}

pub fn draw(e: Option<RenderedEl<Payload>>) {
    clear_screen();
    draw_element(e, 0);
}

fn clear_screen() {
    println!("\x1b[0;0H\x1b[2J");
}

fn draw_element(e: Option<RenderedEl<Payload>>, level: usize) {
    match e {
        None => {}
        Some(rel) => {
            println!("{}{}", "    ".repeat(level), rel.payload.text);
            for ch in rel.children {
                draw_element(ch, level + 1);
            }
        }
    }
}
