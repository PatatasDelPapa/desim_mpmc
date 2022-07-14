use std::{rc::Rc, cell::Cell};

use desim::{SimGen, Effect, ProcessId};

use crate::state::{State, StateKey};

pub struct PassivatedList {
    producers: Vec<(ProcessId, Passivated)>,
    consumers: Vec<(ProcessId, Passivated)>,
}

#[derive(PartialEq, Clone, Copy)]
enum Passivated {
    True,
    False,
    Warned,
}

pub fn producer(shared_state: Rc<Cell<State>>, passivated_key: StateKey<PassivatedList>, consumer_key: StateKey<Vec<usize>>, count_key: StateKey<usize>, self_id: StateKey<usize>) -> Box<SimGen<Effect>> {
    let mut state = shared_state.take();
    
    let self_id = *state.get(self_id).unwrap();
    
    shared_state.set(state);
    todo!()
}

pub fn consumer(shared_state: Rc<Cell<State>>, passivated_key: StateKey<PassivatedList>, consumer_key: StateKey<Vec<usize>>, count_key: StateKey<usize>, self_id: StateKey<usize>) -> Box<SimGen<Effect>> {
    todo!()
}