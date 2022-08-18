#![feature(generators, generator_trait)]
#![allow(unused)]
use std::{cell::Cell, rc::Rc};

use desim::{Simulation, Effect};
use models::{producer, PassivatedList, consumer, Passivated};
use state::State;

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

    let producer_key_1 = state.insert(usize::MAX);
    let consumer_key_1 = state.insert(usize::MAX);

    let producer = simulation.create_process(producer(
        Rc::clone(&shared_state),
        passivated_list,
        count_key,
        producer_key_1,
    ));

    let consumer = simulation.create_process(consumer(
        Rc::clone(&shared_state),
        passivated_list,
        count_key,
        consumer_key_1,
    ));
    

    let producer_key_1 = state.get_mut(producer_key_1).unwrap();
    *producer_key_1 = producer;

    let consumer_key_1 = state.get_mut(consumer_key_1).unwrap();
    *consumer_key_1 = consumer;

    let passivated_list = state.get_mut(passivated_list).unwrap();
    *passivated_list = PassivatedList::new(vec![(producer, Passivated::False)], vec![(consumer, Passivated::False)]);

    shared_state.set(state);
    simulation
}