use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub enum El<TPayload> {
    None,
    Node(Node<TPayload, El<TPayload>>),
    Component(Box<dyn Component<TPayload>>),
}

pub struct RenderedEl<TPayload> {
    pub path: String,
    pub payload: TPayload,
    pub children: Vec<Option<RenderedEl<TPayload>>>,
}

pub struct Node<TPayload, TChild> {
    pub payload: TPayload,
    pub children: Vec<TChild>,
}

impl<TPayload, TChild> Node<TPayload, TChild> {
    pub fn new(payload: TPayload) -> Self {
        Self {
            payload,
            children: Vec::new(),
        }
    }

    pub fn add_child(mut self, e: TChild) -> Self {
        self.children.push(e);
        self
    }

    pub fn add_children(mut self, mut cn: Vec<TChild>) -> Self {
        cn.drain(..).for_each(|c| self.children.push(c));
        self
    }
}

// Automatically implemented by macro for
// all structs implementing trait Component
pub trait KnowsType<TPayload> {
    fn type_id(&self) -> std::any::TypeId;
}

impl<T: 'static, U: 'static> KnowsType<U> for T
where
    T: Component<U>,
    U: Clone,
{
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }
}

pub trait Component<TPayload>: KnowsType<TPayload> {
    fn initial_state(&self) -> Rc<dyn Any> {
        Rc::new(0)
    }

    fn render(&self, state: Rc<dyn Any>, set_state: &mut dyn FnMut(Rc<dyn Any>)) -> El<TPayload>;
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

// Exposed to Components; component path is curried
pub type SetState<'a> = &'a mut dyn FnMut(Rc<dyn Any>);

// ////////////////////////////////////////////////////////////////////////////
pub trait StatefulComponent<T> {}
pub trait StatefulComponentCast<T: 'static> {
    fn cast_state_from_any(&self, state: Rc<dyn Any>) -> Rc<T>;
    fn state_from_any(&self, state: Rc<dyn Any>) -> T;
}

impl<U: 'static, T> StatefulComponentCast<U> for T
where
    T: StatefulComponent<U>,
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

fn render<TPayload: 'static>(
    el: &El<TPayload>,
    path: &str,
    sibling_num: usize,
    state_store: &mut StateStore,
) -> Option<RenderedEl<TPayload>>
where
    TPayload: Clone,
{
    match el {
        El::Node(n) => render_node(
            n,
            &format!("{}/{}~Node", path, sibling_num),
            sibling_num,
            state_store,
        ),
        El::Component(c) => render_stateful_component(
            c,
            &format!("{}/{}~{:?}", path, sibling_num, c.type_id()),
            sibling_num,
            state_store,
        ),
        El::None => None,
    }
}

fn render_node<TPayload: 'static>(
    n: &Node<TPayload, El<TPayload>>,
    path: &str,
    _sibling_num: usize,
    state_store: &mut StateStore,
) -> Option<RenderedEl<TPayload>>
where
    TPayload: Clone,
{
    let mut children: Vec<Option<RenderedEl<TPayload>>> = Vec::new();

    if !n.children.is_empty() {
        for i in 0..n.children.len() {
            children.push(render(&n.children[i], path, i, state_store));
        }
    }

    Some(RenderedEl {
        path: String::from(path),
        payload: n.payload.clone(),
        children,
    })
}

#[allow(clippy::borrowed_box)]
fn render_stateful_component<TPayload: 'static>(
    c: &Box<dyn Component<TPayload>>,
    path: &str,
    sibling_num: usize,
    state_store: &mut StateStore,
) -> Option<RenderedEl<TPayload>>
where
    TPayload: Clone,
{
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

pub fn run_app<TPayload: 'static>(
    el: &El<TPayload>,
    state_store: &mut StateStore,
) -> Option<RenderedEl<TPayload>>
where
    TPayload: Clone,
{
    render(el, "", 0, state_store)
}
