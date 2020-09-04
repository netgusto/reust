use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub enum El<'a, TPayload: 'a> {
    None,
    Node(Node<TPayload, El<'a, TPayload>>),
    Component(Box<dyn Component<'a, TPayload> + 'a>),
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

// impl<T: 'static, U: 'static> KnowsType<U> for T
// where
//     T: Component<U>,
// {
//     fn type_id(&self) -> std::any::TypeId {
//         std::any::TypeId::of::<T>()
//     }
// }

use crate::component::counter::CounterComponentState;

pub trait Component<'a, TPayload> {
    fn initial_state(&self) -> Rc<CounterComponentState> {
        Rc::new(CounterComponentState { num: 0 })
    }

    fn render(
        &self,
        state: Rc<CounterComponentState>,
        set_state: &'a mut dyn FnMut(Rc<CounterComponentState>),
    ) -> El<TPayload>;
}

#[derive(Debug)]
pub struct StateStore {
    state: HashMap<String, Rc<CounterComponentState>>,
}

impl StateStore {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn set(&mut self, path: &str, state: Rc<CounterComponentState>) {
        self.state.insert(String::from(path), state);
    }

    pub fn get(&self, path: &str) -> Option<Rc<CounterComponentState>> {
        match self.state.get(path) {
            None => None,
            Some(s) => Some(Rc::clone(s)),
        }
    }
}

// Exposed to Components; component path is curried
pub type SetState<'a> = &'a mut dyn FnMut(Rc<CounterComponentState>);

// ////////////////////////////////////////////////////////////////////////////
// pub trait StatefulComponent<T> {}
// pub trait StatefulComponentCast<T: 'static> {
//     fn cast_state_from_any(&self, state: Rc<dyn Any>) -> Rc<T>;
//     fn state_from_any(&self, state: Rc<dyn Any>) -> T;
// }

// impl<U: 'static, T> StatefulComponentCast<U> for T
// where
//     T: StatefulComponent<U>,
//     U: Clone,
// {
//     fn cast_state_from_any(&self, state: Rc<dyn Any>) -> Rc<U> {
//         match state.downcast::<U>() {
//             Ok(u) => u,
//             Err(_) => panic!("NOOOO"),
//         }
//     }

//     fn state_from_any(&self, state: Rc<dyn Any>) -> U {
//         let x = self.cast_state_from_any(state);
//         (*x).clone()
//     }
// }
// ////////////////////////////////////////////////////////////////////////////

fn render<'a, TPayload: 'a>(
    el: &'a El<'a, TPayload>,
    path: &'a str,
    sibling_num: usize,
    state_store: &'a mut StateStore,
) -> Option<RenderedEl<TPayload>> {
    match el {
        El::Node(n) => render_node(
            n,
            &format!("{}/{}~Node", path, sibling_num),
            sibling_num,
            state_store,
        ),
        El::Component(c) => render_stateful_component(
            c,
            &format!("{}/{}~{:?}", path, sibling_num, "ZETYPE"),
            sibling_num,
            state_store,
        ),
        El::None => None,
    }
}

fn render_node<'a, TPayload: 'a>(
    n: &'a Node<TPayload, El<'a, TPayload>>,
    path: &'a str,
    _sibling_num: usize,
    state_store: &'a mut StateStore,
) -> Option<RenderedEl<TPayload>> {
    let mut children: Vec<Option<RenderedEl<TPayload>>> = Vec::new();

    if !n.children.is_empty() {
        children.push(render(&n.children[0], path, 0, state_store));
        // for i in 0..n.children.len() {
        //     children.push(render(&n.children[i], path, i, state_store));
        // }
    }

    Some(RenderedEl {
        path: String::from(path),
        payload: Rc::clone(&n.payload),
        children,
    })
}

#[allow(clippy::borrowed_box)]
fn render_stateful_component<'a, TPayload: 'a>(
    c: &'a Box<dyn Component<'a, TPayload> + 'a>,
    path: &'a str,
    sibling_num: usize,
    state_store: &'a mut StateStore,
) -> Option<RenderedEl<TPayload>> {
    let s = match state_store.get(path) {
        None => {
            let initial_state = c.initial_state();
            state_store.set(path, initial_state.clone());
            initial_state
        }
        Some(s) => s,
    };

    let mut set_state = |s: Rc<CounterComponentState>| state_store.set(path, s);

    render(&c.render(s, &mut set_state), path, sibling_num, state_store)
}

pub fn run_app<'a, TPayload: 'a>(
    el: &'a El<'a, TPayload>,
    state_store: &'a mut StateStore,
) -> Option<RenderedEl<TPayload>> {
    render(el, "", 0, state_store)
}
