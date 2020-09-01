use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub enum El {
    None,
    Node(Node<El>),
    StatefulComponent(Box<dyn StatefulComponent>),
}

#[allow(dead_code)]
struct RenderedNode {
    path: String,
    el: Node<Option<RenderedNode>>,
}

pub struct Node<TChild> {
    text: String,
    children: Vec<TChild>,
}

impl<TChild> Node<TChild> {
    pub fn new(s: &str) -> Self {
        Self {
            text: String::from(s),
            children: Vec::new(),
        }
    }

    pub fn add_child(mut self, e: TChild) -> Self {
        self.children.push(e);
        self
    }
}

// Automatically implemented by macro for
// all structs implementing trait StatefulComponent
pub trait KnowsType {
    fn type_id(&self) -> std::any::TypeId;
}

impl<T: 'static> KnowsType for T
where
    T: StatefulComponent,
{
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }
}

pub trait StatefulComponent: KnowsType {
    fn initial_state(&self) -> Rc<dyn Any>;
    fn render(&self, state: Rc<dyn Any>, set_state: &mut dyn FnMut(Rc<dyn Any>)) -> El;
}

#[derive(Debug)]
pub struct StateStore {
    state: HashMap<String, Rc<dyn Any>>,
}

impl StateStore {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn set(&mut self, path: &str, state: Rc<dyn Any>) {
        self.state.insert(String::from(path), state);
    }

    pub fn get(&self, path: &str) -> Option<Rc<dyn Any>> {
        match self.state.get(path) {
            None => None,
            Some(s) => Some(Rc::clone(s)),
        }
    }
}

// Exposed to StatefulComponents; component path is curried
pub type SetState<'a> = &'a mut dyn FnMut(Rc<dyn Any>);

// ////////////////////////////////////////////////////////////////////////////
pub trait StateCast<T> {}
pub trait StateCastPatched<T: 'static> {
    fn cast_state_from_any(&self, state: Rc<dyn Any>) -> Rc<T>;
    fn state_from_any(&self, state: Rc<dyn Any>) -> T;
}

impl<U: 'static, T> StateCastPatched<U> for T
where
    T: StateCast<U>,
    U: Clone,
{
    fn cast_state_from_any(&self, state: Rc<dyn Any>) -> Rc<U> {
        match state.downcast::<U>() {
            Ok(u) => u,
            Err(_) => panic!("NOOOO"),
        }
    }

    fn state_from_any(&self, state: Rc<dyn Any>) -> U {
        let x = self.cast_state_from_any(state);
        (*x).clone()
    }
}
// ////////////////////////////////////////////////////////////////////////////

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

    let mut set_state = |s: Rc<dyn Any>| state_store.set(path, s);

    render(&c.render(s, &mut set_state), path, sibling_num, state_store)
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

pub fn run_app(el: &El, state_store: &mut StateStore) {
    draw(render(el, "", 0, state_store), 0);
}
