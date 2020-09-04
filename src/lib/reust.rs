use std::any::Any;
use std::cell::RefCell;
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
    pub payload: Rc<TPayload>,
    pub children: Vec<Option<RenderedEl<TPayload>>>,
}

pub struct Node<TPayload, TChild> {
    pub payload: Rc<TPayload>,
    pub children: Vec<TChild>,
}

impl<TPayload, TChild> Node<TPayload, TChild> {
    pub fn new(payload: TPayload) -> Self {
        Self {
            payload: Rc::new(payload),
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
{
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }
}

pub trait Component<TPayload>: KnowsType<TPayload> {
    fn initial_state(&self) -> Rc<dyn Any> {
        Rc::new(0)
    }

    fn render<'a>(
        &self,
        state: Rc<dyn Any + 'a>,
        set_state: Box<dyn Fn(Rc<dyn Any>)>,
    ) -> El<TPayload>;
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

// ////////////////////////////////////////////////////////////////////////////
pub trait StatefulComponent<T> {}
pub trait StatefulComponentCast<T: 'static> {
    fn must_cast_state_rc_from_any(&self, state: Rc<dyn Any>) -> Rc<T>;
    fn must_receive_state(&self, state: Rc<dyn Any>) -> T;
    fn cast_state_rc_from_any(&self, state: Rc<dyn Any>) -> Result<Rc<T>, Rc<dyn Any>>;
    fn receive_state(&self, state: Rc<dyn Any>) -> Result<T, Rc<dyn Any>>;
}

impl<U: 'static, T> StatefulComponentCast<U> for T
where
    T: StatefulComponent<U>,
    U: Clone,
{
    fn must_cast_state_rc_from_any(&self, state: Rc<dyn Any>) -> Rc<U> {
        match self.cast_state_rc_from_any(state) {
            Ok(u) => u,
            Err(_) => panic!("must_cast_state_rc_from_any: could not cast state to expected type; this is a programming error"),
        }
    }

    fn must_receive_state(&self, state: Rc<dyn Any>) -> U {
        let cast = self.must_cast_state_rc_from_any(state);
        (*cast).clone()
    }

    fn cast_state_rc_from_any(&self, state: Rc<dyn Any>) -> Result<Rc<U>, Rc<dyn Any>> {
        state.downcast::<U>()
    }

    fn receive_state(&self, state: Rc<dyn Any>) -> Result<U, Rc<dyn Any>> {
        match self.cast_state_rc_from_any(state) {
            Ok(u) => Ok((*u).clone()),
            Err(e) => Err(e),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////

fn render<TPayload: 'static>(
    el: &El<TPayload>,
    path: String,
    sibling_num: usize,
    state_store: Rc<RefCell<StateStore>>,
) -> Option<RenderedEl<TPayload>> {
    match el {
        El::Node(n) => render_node(
            n,
            format!("{}/{}~Node", path, sibling_num),
            sibling_num,
            state_store,
        ),
        El::Component(c) => render_stateful_component(
            c,
            format!("{}/{}~{:?}", path, sibling_num, c.type_id()),
            sibling_num,
            state_store,
        ),
        El::None => None,
    }
}

fn render_node<TPayload: 'static>(
    n: &Node<TPayload, El<TPayload>>,
    path: String,
    _sibling_num: usize,
    state_store: Rc<RefCell<StateStore>>,
) -> Option<RenderedEl<TPayload>> {
    let mut children: Vec<Option<RenderedEl<TPayload>>> = Vec::new();

    if !n.children.is_empty() {
        for i in 0..n.children.len() {
            children.push(render(
                &n.children[i],
                path.clone(),
                i,
                Rc::clone(&state_store),
            ));
        }
    }

    Some(RenderedEl {
        path,
        payload: Rc::clone(&n.payload),
        children,
    })
}

#[allow(clippy::borrowed_box)]
fn render_stateful_component<TPayload: 'static>(
    c: &Box<dyn Component<TPayload>>,
    path: String,
    sibling_num: usize,
    state_store: Rc<RefCell<StateStore>>,
) -> Option<RenderedEl<TPayload>> {
    let state_store_clone = Rc::clone(&state_store);
    let path_clone = path.clone();

    let set_state = Box::new(move |s: Rc<dyn Any>| {
        state_store_clone
            .borrow_mut()
            .set(path_clone.as_str(), Rc::clone(&s))
    });

    let s = match state_store.borrow().get(path.as_str()) {
        None => c.initial_state(),
        Some(s) => s,
    };

    render(&c.render(s, set_state), path, sibling_num, state_store)
}

pub fn run_app<TPayload: 'static>(
    el: &El<TPayload>,
    state_store: Rc<RefCell<StateStore>>,
) -> Option<RenderedEl<TPayload>> {
    render(el, "".to_string(), 0, state_store)
}
