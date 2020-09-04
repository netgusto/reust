use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub enum El<TPayload> {
    None,
    Node(Node<TPayload, El<TPayload>>),
    Component(Box<dyn Component<TPayload>>),
    Container(Vec<El<TPayload>>),
}

pub enum RenderedEl<TPayload> {
    None,
    Node(RenderedNode<TPayload>),
    Container(Vec<RenderedEl<TPayload>>),
}

pub struct RenderedNode<TPayload> {
    pub path: String,
    pub payload: Rc<TPayload>,
    pub children: Vec<RenderedEl<TPayload>>,
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

    #[allow(dead_code)]
    pub fn add_child(mut self, e: TChild) -> Self {
        self.children.push(e);
        self
    }

    #[allow(dead_code)]
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

pub type BoxedState = dyn Any;
pub type SetState = dyn Fn(Rc<BoxedState>);

pub trait Component<TPayload>: KnowsType<TPayload> {
    fn initial_state(&self) -> Rc<dyn Any> {
        Rc::new(0)
    }

    fn render(&self, state: Rc<BoxedState>, set_state: Rc<SetState>) -> El<TPayload>;
}

#[derive(Debug)]
pub struct StateStore {
    state: HashMap<String, Rc<dyn Any>>,
}

pub fn new_state_store() -> Rc<RefCell<StateStore>> {
    Rc::new(RefCell::new(StateStore::new()))
}

impl StateStore {
    fn new() -> Self {
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
pub trait StateReceiver<T> {}
pub trait StateReceiverDefault<T: 'static> {
    fn must_receive_state_rc(&self, state: Rc<dyn Any>) -> Rc<T>;
    fn must_receive_state(&self, state: Rc<dyn Any>) -> T;
    fn receive_state_rc(&self, state: Rc<dyn Any>) -> Result<Rc<T>, Rc<dyn Any>>;
    fn receive_state(&self, state: Rc<dyn Any>) -> Result<T, Rc<dyn Any>>;
}

impl<U: 'static, T> StateReceiverDefault<U> for T
where
    T: StateReceiver<U>,
    U: Clone,
{
    fn must_receive_state_rc(&self, state: Rc<dyn Any>) -> Rc<U> {
        match self.receive_state_rc(state) {
            Ok(u) => u,
            Err(_) => panic!("StateReceiver.must_receive_state_rc: could not cast state to expected type; this is a programming error"),
        }
    }

    fn must_receive_state(&self, state: Rc<dyn Any>) -> U {
        let cast = self.must_receive_state_rc(state);
        (*cast).clone()
    }

    fn receive_state_rc(&self, state: Rc<dyn Any>) -> Result<Rc<U>, Rc<dyn Any>> {
        state.downcast::<U>()
    }

    fn receive_state(&self, state: Rc<dyn Any>) -> Result<U, Rc<dyn Any>> {
        match self.receive_state_rc(state) {
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
) -> RenderedEl<TPayload> {
    match el {
        El::None => RenderedEl::None,
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
        El::Container(c) => render_container(
            c,
            format!("{}/{}~Container", path, sibling_num),
            sibling_num,
            state_store,
        ),
    }
}

fn render_container<TPayload: 'static>(
    cont: &[El<TPayload>],
    path: String,
    _sibling_num: usize,
    state_store: Rc<RefCell<StateStore>>,
) -> RenderedEl<TPayload> {
    let mut container: Vec<RenderedEl<TPayload>> = Vec::new();
    for (sibling_num, ch) in cont.iter().enumerate() {
        container.push(render(
            ch,
            path.clone(),
            sibling_num,
            Rc::clone(&state_store),
        ))
    }

    RenderedEl::Container(container)
}

fn render_node<TPayload: 'static>(
    n: &Node<TPayload, El<TPayload>>,
    path: String,
    _sibling_num: usize,
    state_store: Rc<RefCell<StateStore>>,
) -> RenderedEl<TPayload> {
    let mut children: Vec<RenderedEl<TPayload>> = Vec::new();

    for i in 0..n.children.len() {
        children.push(render(
            &n.children[i],
            path.clone(),
            i,
            Rc::clone(&state_store),
        ));
    }

    RenderedEl::Node(RenderedNode {
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
) -> RenderedEl<TPayload> {
    let state_store_clone = Rc::clone(&state_store);
    let path_clone = path.clone();

    let set_state = Rc::new(move |s: Rc<dyn Any>| {
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

pub fn render_app<TPayload: 'static>(
    el: &El<TPayload>,
    state_store: Rc<RefCell<StateStore>>,
) -> RenderedEl<TPayload> {
    render(el, "".to_string(), 0, state_store)
}
