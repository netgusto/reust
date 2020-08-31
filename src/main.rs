use std::any::Any;
use std::collections::HashMap;
use std::{thread::sleep, time::Duration};

#[allow(dead_code)]
enum El {
    None,
    Node(Node<El>),
    StatefulComponent(Box<dyn StatefulComponent>),
}

#[allow(dead_code)]
struct RenderedNode {
    path: String,
    el: Node<Option<RenderedNode>>,
}

struct Node<TChild> {
    text: String,
    children: Vec<TChild>,
}

impl<TChild> Node<TChild> {
    fn new(s: &str) -> Self {
        Self {
            text: String::from(s),
            children: Vec::new(),
        }
    }

    fn add_child(mut self, e: TChild) -> Self {
        self.children.push(e);
        self
    }
}

// Automatically implemented by macro for
// all structs implementing trait StatefulComponent
trait KnowsType {
    fn type_id(&self) -> std::any::TypeId;
}

trait StatefulComponent: KnowsType {
    fn initial_state(&self) -> GenericState {
        GenericState {
            ..Default::default()
        }
    }

    fn render(&self, state: Option<GenericState>, set_state: &mut dyn FnMut(GenericState)) -> El;
}

#[derive(Debug, Default, Clone)]
struct GenericState {
    num: i32,
    text: String,
}

#[derive(Debug)]
struct StateStore {
    state: HashMap<String, GenericState>,
}

impl StateStore {
    fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn set(&mut self, path: &str, state: GenericState) {
        self.state.insert(String::from(path), state);
    }

    pub fn get(&self, path: &str) -> Option<GenericState> {
        match self.state.get(path) {
            None => None,
            Some(s) => Some(s.clone()),
        }
    }
}

// Exposed to StatefulComponents; component path is curried
type SetState<'a> = &'a mut dyn FnMut(GenericState);

// ////////////////////////////////////////////////////////////////////////////
macro_rules! impl_patch_trait {
    () => {
        impl<T: 'static> KnowsType for T
        where
            T: StatefulComponent,
        {
            fn type_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<T>()
            }
        }
    };
}
impl_patch_trait!();
// ////////////////////////////////////////////////////////////////////////////
trait StateCast<T> {}
trait StateCastPatched<T: 'static> {
    fn print_state_type(&self);
    fn cast_state_from_any(&self, state: Box<dyn Any>) -> Box<T>;
}

macro_rules! impl_patch_statecaster {
    () => {
        impl<U: 'static, T> StateCastPatched<U> for T
        where
            T: StateCast<U>,
        {
            fn print_state_type(&self) {
                println!("THE STATE TYPE IS {:?}", std::any::TypeId::of::<U>());
            }

            fn cast_state_from_any(&self, state: Box<dyn Any>) -> Box<U> {
                match state.downcast::<U>() {
                    Ok(u) => u,
                    Err(_) => panic!("NOOOO"),
                }
            }
        }
    };
}
impl_patch_statecaster!();

// ////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut state = StateStore::new();
    {
        loop {
            let app = El::Node(
                Node::new("Root")
                    .add_child(El::Node(Node::new("# Header A")))
                    .add_child(El::StatefulComponent(Box::new(CounterComponent {
                        initial_counter: 26,
                    })))
                    .add_child(El::Node(
                        Node::new("----------------------------").add_child(El::StatefulComponent(
                            Box::new(CounterComponent {
                                initial_counter: -80,
                            }),
                        )),
                    ))
                    .add_child(El::Node(Node::new("Ctrl-c to quit"))),
            );

            clear_screen();
            draw(render(&app, "", 0, &mut state), 0);
            sleep(Duration::from_millis(1000));
        }
    }
}

fn clear_screen() {
    println!("\x1b[0;0H\x1b[2J");
}

fn render(
    el: &El,
    path: &str,
    sibling_num: usize,
    state_store: &mut StateStore,
) -> Option<RenderedNode> {
    match el {
        El::Node(n) => render_node(
            n,
            &format!("{}/{}~Node", path, sibling_num),
            sibling_num,
            state_store,
        ),
        El::StatefulComponent(c) => render_stateful_component(
            c,
            &format!("{}/{}~{:?}", path, sibling_num, c.type_id()),
            sibling_num,
            state_store,
        ),
        El::None => None,
    }
}

fn render_node(
    n: &Node<El>,
    path: &str,
    _sibling_num: usize,
    state_store: &mut StateStore,
) -> Option<RenderedNode> {
    let mut new_n = Node::new(&n.text);

    if !n.children.is_empty() {
        for i in 0..n.children.len() {
            new_n
                .children
                .push(render(&n.children[i], path, i, state_store));
        }
    }

    Some(RenderedNode {
        path: String::from(path),
        el: new_n,
    })
}

#[allow(clippy::borrowed_box)]
fn render_stateful_component(
    c: &Box<dyn StatefulComponent>,
    path: &str,
    sibling_num: usize,
    state_store: &mut StateStore,
) -> Option<RenderedNode> {
    let s = match state_store.get(path) {
        None => {
            let initial_state = c.initial_state();
            state_store.set(path, initial_state.clone());
            initial_state
        }
        Some(s) => s,
    };

    let mut set_state = |s: GenericState| state_store.set(path, s);

    render(
        &c.render(Some(s), &mut set_state),
        path,
        sibling_num,
        state_store,
    )
}

fn draw(e: Option<RenderedNode>, level: usize) {
    match e {
        None => {}
        Some(n) => {
            println!("{}{}", "    ".repeat(level), n.el.text);
            for ch in n.el.children {
                draw(ch, level + 1);
            }
        }
    }
}

// /////////////////////////////////
// CounterComponent

struct CounterComponent {
    initial_counter: i32,
}

// struct CounterComponentState {
//     num: i32,
// }

// impl StateCast<CounterComponentState> for CounterComponent {}
impl StatefulComponent for CounterComponent {
    fn initial_state(&self) -> GenericState {
        GenericState {
            text: String::from(""),
            num: self.initial_counter,
        }
    }

    fn render(&self, state: Option<GenericState>, set_state: SetState) -> El {
        let counter = match state {
            None => -1,
            Some(s) => {
                set_state(GenericState {
                    num: s.num + 1,
                    text: s.text,
                });
                s.num
            }
        };

        if counter % 10 == 0 {
            El::Node(Node::new(&format!("{} IS %10!; skipping!", counter)))
        } else {
            El::Node(
                Node::new(&format!("The counter is: {}", counter)).add_child(El::Node(
                    Node::new("Sub element").add_child(El::StatefulComponent(Box::new(
                        LeafComponent {
                            over_100: counter > 100,
                        },
                    ))),
                )),
            )
        }
    }
}

// /////////////////////////////////
// LeafComponent

struct LeafComponent {
    over_100: bool,
}

impl StatefulComponent for LeafComponent {
    fn render(&self, _: Option<GenericState>, _set_state: SetState) -> El {
        if self.over_100 {
            El::Node(Node::new("It's OVER 9000! (jk 100)"))
        } else {
            El::Node(Node::new("The leaf"))
        }
    }
}
