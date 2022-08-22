#![feature(generators, generator_trait)]
#![allow(unused)]
use std::{cell::Cell, rc::Rc, collections::VecDeque};

use desim::{Simulation, Effect};
use models::{producer, PassivatedList, consumer};
use state::{State, StateKey};

mod models;
mod state;

fn main() {
    println!("Hello, world!");
}

pub fn simulation(limit: f64) {
    let simulation = set_simulation();
    simulation.run(desim::EndCondition::Time(limit));
}

pub fn set_simulation() -> Simulation<Effect> {
    let mut simulation = Simulation::new();
    let shared_state = Rc::new(Cell::new(State::default()));

    let mut state = shared_state.take();
    let count_key = state.insert(0);

    let passivated_list = PassivatedList::default();
    let passivated_list = state.insert(passivated_list);

    shared_state.set(state);

    producer_factory(
        Rc::clone(&shared_state),
        5,
        count_key,
        passivated_list,
        &mut simulation,
    );
    
    consumer_factory(
        Rc::clone(&shared_state),
        6,
        count_key,
        passivated_list,
        &mut simulation,
    );
        
    simulation
}

fn producer_factory(
    shared_state: Rc<Cell<State>>, 
    quantity: u8, 
    count_key: StateKey<usize>, 
    passivated_list: StateKey<PassivatedList>,
    simulation: &mut Simulation<Effect>,
) {
    let mut state = shared_state.take();
    let producer_key = state.insert(usize::MAX);
    for _ in 0..quantity {
        let producer_key = state.insert(usize::MAX);
        let producer = simulation.create_process(producer(
            Rc::clone(&shared_state),
            passivated_list,
            count_key,
            producer_key,
        ));  
        let producer_id = state.get_mut(producer_key).unwrap();
        *producer_id = producer;
        simulation.schedule_event(0.0, producer, Effect::TimeOut(0.0));
    }
    shared_state.set(state);
}

fn consumer_factory(
    shared_state: Rc<Cell<State>>, 
    quantity: u8, 
    count_key: StateKey<usize>, 
    passivated_list: StateKey<PassivatedList>,
    simulation: &mut Simulation<Effect>,
) {
    let mut state = shared_state.take();
    let consumer_key = state.insert(usize::MAX);
    for _ in 0..quantity {
        let consumer_key = state.insert(usize::MAX);
        let consumer = simulation.create_process(consumer(
            Rc::clone(&shared_state),
            passivated_list,
            count_key,
            consumer_key,
        ));  
        let consumer_id = state.get_mut(consumer_key).unwrap();
        *consumer_id = consumer;
        simulation.schedule_event(0.0, consumer, Effect::TimeOut(0.0));
    }
    shared_state.set(state);
}