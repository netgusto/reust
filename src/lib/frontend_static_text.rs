/*
use super::reust::{El, Node, RenderedEl};

#[derive(Clone)]
pub struct TextNode {
    pub text: String,
}

impl TextNode {
    #[allow(dead_code)]
    pub fn new(text: &str) -> Self {
        Self {
            text: String::from(text),
        }
    }
}

#[allow(dead_code)]
pub fn node(text: &str) -> Node<TextNode, El<TextNode>> {
    Node::new(TextNode::new(text))
}

#[allow(dead_code)]
pub fn draw(e: Option<RenderedEl<TextNode>>) {
    clear_screen();
    draw_element(e, 0);
}

fn clear_screen() {
    println!("\x1b[0;0H\x1b[2J");
}

fn draw_element(e: Option<RenderedEl<TextNode>>, level: usize) {
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
*/
