use crate::prelude::*;

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
pub fn draw_graph(e: RenderedEl<TextNode>) {
    clear_screen();
    draw_element(e, 0);
}

fn clear_screen() {
    println!("\x1b[0;0H\x1b[2J");
}

fn draw_element(e: RenderedEl<TextNode>, level: usize) {
    match e {
        RenderedEl::None => {}
        RenderedEl::Node(rel) => {
            println!("{}{}", "    ".repeat(level), rel.payload.text);
            for ch in rel.children {
                draw_element(ch, level + 1);
            }
        }
        RenderedEl::Container(cont) => {
            for ch in cont {
                draw_element(ch, level);
            }
        }
    }
}
