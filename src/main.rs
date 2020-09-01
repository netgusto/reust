use std::any::Any;
use std::rc::Rc;
use std::{thread::sleep, time::Duration};

mod lib;
use lib::{
    run_app, El, Node, SetState, StateCast, StateCastPatched, StateStore, StatefulComponent,
};

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
            run_app(&app, &mut state);
            sleep(Duration::from_millis(1000));
        }
    }
}

fn clear_screen() {
    println!("\x1b[0;0H\x1b[2J");
}

// /////////////////////////////////
// CounterComponent

struct CounterComponent {
    initial_counter: i32,
}

#[derive(Clone)]
struct CounterComponentState {
    num: i32,
}

impl StateCast<CounterComponentState> for CounterComponent {}
impl StatefulComponent for CounterComponent {
    fn initial_state(&self) -> Rc<dyn Any> {
        Rc::new(CounterComponentState {
            num: self.initial_counter,
        })
    }

    fn render(&self, state: Rc<dyn Any>, set_state: SetState) -> El {
        let mut new_state = self.state_from_any(state);
        new_state.num += 1;
        let counter = new_state.num;
        set_state(Rc::new(new_state));

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
    fn initial_state(&self) -> Rc<dyn Any> {
        Rc::new(0)
    }

    fn render(&self, _: Rc<dyn Any>, _set_state: SetState) -> El {
        if self.over_100 {
            El::Node(Node::new("It's OVER 9000! (jk 100)"))
        } else {
            El::Node(Node::new("The leaf"))
        }
    }
}
