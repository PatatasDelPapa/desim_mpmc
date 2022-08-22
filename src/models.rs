use std::{rc::Rc, cell::Cell, cmp::{Ord, Ordering}, collections::VecDeque};

use desim::{SimGen, Effect, ProcessId};

use crate::state::{State, StateKey};

#[derive(Default, Debug)]
pub struct PassivatedList {
    producers: VecDeque<ProcessId>,
    consumers: VecDeque<ProcessId>,
}

impl PassivatedList {
    pub fn new(producers: VecDeque<ProcessId>, consumers: VecDeque<ProcessId>) -> Self {
        Self {
            producers,
            consumers,
        }
    }
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
            // let passivated_list = &mut state.get_mut(passivated_key).unwrap();
            // let maybe_passivated = passivated_list.producers.iter_mut().find(|(_, state)| *state == Passivated::Hack);
            // if let Some(&mut (passivated_id, _)) = maybe_passivated {
            //     // Convert self to HACK and send Activate
            //     let self_state = passivated_list.consumers.iter_mut().find(|(id, _)| *id == self_id).map(|(_, state)| state).unwrap();
            //     *self_state = Passivated::Hack;
            //     shared_state.set(state);
            //     yield Effect::Event { time: 0.0, process: passivated_id };
            //     let mut state = shared_state.take();
            //     let passivated_list = &mut state.get_mut(passivated_key).unwrap();
            //     let self_state = passivated_list.consumers.iter_mut().find(|(id, _)| *id == self_id).map(|(_, state)| state).unwrap();
            //     *self_state = Passivated::False;
            // } else {
            //     shared_state.set(state);
            // }
            ////////////////////////////////////
            // -------- END HACK CHECK ---------
            ////////////////////////////////////
            let mut state = shared_state.take();
            let count = state.get_mut(count_key).unwrap();
            match (*count).cmp(&limit)  {
                Ordering::Less => {
                    *count += produce_amount;
                    shared_state.set(state);
                    yield Effect::Event { time: hold_time, process: self_id };
                },
                Ordering::Equal | 
                Ordering::Greater => {
                    // Check if someone is in Passivate
                    // If true then Change to Passivate and send Event
                    // else change to Passivate and do Wait
                    let passivated_list = state.get_mut(passivated_key).unwrap();
                    passivated_list.producers.push_back(self_id);
                    
                    if let Some(consumer_id) = passivated_list.consumers.pop_front() {
                        shared_state.set(state);
                        yield Effect::Event { time: 0.0,  process: consumer_id  };
                    } else {
                        shared_state.set(state);
                        yield Effect::Wait;
                    }
                }
            }
        }
    })
}

pub fn consumer(shared_state: Rc<Cell<State>>, passivated_key: StateKey<PassivatedList>, count_key: StateKey<usize>, self_id: StateKey<usize>) -> Box<SimGen<Effect>> {
    let consume_amount = 5;
    let hold_time = 4.0;
    let state = shared_state.take();
    let self_id = *state.get(self_id).unwrap();
    shared_state.set(state);
    Box::new(move |_| {
        loop {
            //////////////////////////////////////
            // -------- BEGIN HACK CHECK ---------
            //////////////////////////////////////
            // let passivated_list = &mut state.get_mut(passivated_key).unwrap();
            // let maybe_passivated = passivated_list.consumers.iter_mut().find(|(_, state)| *state == Passivated::Hack);
            // if let Some(&mut (passivated_id, _)) = maybe_passivated {
            //     // Convert self to HACK and send Activate
            //     let self_state = passivated_list.producers.iter_mut().find(|(id, _)| *id == self_id).map(|(_, state)| state).unwrap();
            //     *self_state = Passivated::Hack;
            //     shared_state.set(state);
            //     yield Effect::Event { time: 0.0, process: passivated_id };
            //     let mut state = shared_state.take();
            //     let passivated_list = &mut state.get_mut(passivated_key).unwrap();
            //     let self_state = passivated_list.producers.iter_mut().find(|(id, _)| *id == self_id).map(|(_, state)| state).unwrap();
            //     *self_state = Passivated::False;
            // } else {
            //     shared_state.set(state);
            // }
            ////////////////////////////////////
            // -------- END HACK CHECK ---------
            ////////////////////////////////////
            // Take the state out of the Rc<Cell<State>>
            let mut state = shared_state.take();
            let count = state.get_mut(count_key).unwrap();
            match (*count).cmp(&consume_amount) {
                Ordering::Greater | Ordering::Equal => {
                    *count -= consume_amount;
                    shared_state.set(state);
                    yield Effect::Event { time: hold_time, process: self_id };
                }, 
                Ordering::Less => {
                    let passivated_list = state.get_mut(passivated_key).unwrap();
                    passivated_list.producers.push_back(self_id);
                    if let Some(producer_id) = passivated_list.producers.pop_front() {
                        shared_state.set(state);
                        yield Effect::Event { time: 0.0, process: producer_id };
                    } else {
                        shared_state.set(state);
                        yield Effect::Wait;
                    }
                }
            }
        }
    })
}

