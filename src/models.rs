use std::{rc::Rc, cell::Cell, cmp::{Ord, Ordering}};

use desim::{SimGen, Effect, ProcessId};

use crate::state::{State, StateKey};

#[derive(Default, Debug)]
pub struct PassivatedList {
    producers: Vec<(ProcessId, Passivated)>,
    consumers: Vec<(ProcessId, Passivated)>,
}

impl PassivatedList {
    pub fn new(producers: Vec<(ProcessId, Passivated)>, consumers: Vec<(ProcessId, Passivated)>) -> Self {
        Self {
            producers,
            consumers,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Passivated {
    True,
    False,
    Hack,
}

pub fn producer(shared_state: Rc<Cell<State>>, passivated_key: StateKey<PassivatedList>, count_key: StateKey<usize>, self_id: StateKey<usize>) -> Box<SimGen<Effect>> {
    let produce_amount = 1;
    let hold_time = 1.0;
    let limit = 12;
    let state = shared_state.take();
    let self_id = *state.get(self_id).unwrap();
    shared_state.set(state);
    Box::new(move |_| {
        loop {
            // Take the state out of the Rc<Cell<State>>
            let mut state = shared_state.take();
            //////////////////////////////////////
            // -------- BEGIN HACK CHECK ---------
            //////////////////////////////////////
            let passivated_list = &mut state.get_mut(passivated_key).unwrap();
            let maybe_passivated = passivated_list.producers.iter_mut().find(|(_, state)| *state == Passivated::Hack);
            if let Some(&mut (passivated_id, _)) = maybe_passivated {
                // Convert self to HACK and send Activate
                let self_state = passivated_list.consumers.iter_mut().find(|(id, _)| *id == self_id).map(|(_, state)| state).unwrap();
                *self_state = Passivated::Hack;
                shared_state.set(state);
                yield Effect::Event { time: 0.0, process: passivated_id };
                let mut state = shared_state.take();
                let passivated_list = &mut state.get_mut(passivated_key).unwrap();
                let self_state = passivated_list.consumers.iter_mut().find(|(id, _)| *id == self_id).map(|(_, state)| state).unwrap();
                *self_state = Passivated::False;
            } else {
                shared_state.set(state);
            }
            ////////////////////////////////////
            // -------- END HACK CHECK ---------
            ////////////////////////////////////
            let mut state = shared_state.take();
            let count = *state.get(count_key).unwrap();
            match count.cmp(&limit)  {
                Ordering::Less => {
                    todo!()
                },
                Ordering::Equal | 
                Ordering::Greater => {
                    // Check if someone is in Passivate
                    // If true then Change to Passivate and send Event
                    // else change to Passivate and do Wait
                    todo!()
                }
            }
        }
    })
}

pub fn consumer(shared_state: Rc<Cell<State>>, passivated_key: StateKey<PassivatedList>, count_key: StateKey<usize>, self_id: StateKey<usize>) -> Box<SimGen<Effect>> {
    let state = shared_state.take();
    let self_id = *state.get(self_id).unwrap();
    shared_state.set(state);
    Box::new(move |_| {
        loop {
            // Take the state out of the Rc<Cell<State>>
            let mut state = shared_state.take();
            //////////////////////////////////////
            // -------- BEGIN HACK CHECK ---------
            //////////////////////////////////////
            let passivated_list = &mut state.get_mut(passivated_key).unwrap();
            let maybe_passivated = passivated_list.consumers.iter_mut().find(|(_, state)| *state == Passivated::Hack);
            if let Some(&mut (passivated_id, _)) = maybe_passivated {
                // Convert self to HACK and send Activate
                let self_state = passivated_list.producers.iter_mut().find(|(id, _)| *id == self_id).map(|(_, state)| state).unwrap();
                *self_state = Passivated::Hack;
                shared_state.set(state);
                yield Effect::Event { time: 0.0, process: passivated_id };
                let mut state = shared_state.take();
                let passivated_list = &mut state.get_mut(passivated_key).unwrap();
                let self_state = passivated_list.producers.iter_mut().find(|(id, _)| *id == self_id).map(|(_, state)| state).unwrap();
                *self_state = Passivated::False;
            } else {
                shared_state.set(state);
            }
            ////////////////////////////////////
            // -------- END HACK CHECK ---------
            ////////////////////////////////////
        }
    })
}

