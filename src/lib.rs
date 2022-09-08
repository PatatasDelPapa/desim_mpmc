#![feature(generators, generator_trait)]
// #![allow(unused)]
use std::{cell::Cell, rc::Rc};

use desim::{Effect, Simulation};
use models::{consumer, producer, PassivatedList};
use state::{State, StateKey};

mod models;
mod state;

pub fn simulation(limit: f64, producers: u8, consumers: u8) {
    let (simulation, _shared_state, _count_key) = set_simulation(producers, consumers);
    let _simulation = simulation.run(desim::EndCondition::Time(limit));
    // let mut state = shared_state.take();
    // let count = state.remove(count_key).unwrap();
    // println!("Final Results:");
    // println!("\tCount: {}", count);
    // println!("\tTime: {}", simulation.time());
}

pub fn set_simulation(
    producers: u8,
    consumers: u8,
) -> (Simulation<Effect>, Rc<Cell<State>>, StateKey<usize>) {
    let mut simulation = Simulation::new();
    let shared_state = Rc::new(Cell::new(State::default()));

    let mut state = shared_state.take();
    let count_key = state.insert(0);

    let passivated_list = PassivatedList::default();
    let passivated_list = state.insert(passivated_list);

    shared_state.set(state);

    producer_factory(
        Rc::clone(&shared_state),
        producers,
        count_key,
        passivated_list,
        &mut simulation,
    );

    consumer_factory(
        Rc::clone(&shared_state),
        consumers,
        count_key,
        passivated_list,
        &mut simulation,
    );

    (simulation, shared_state, count_key)
}

pub fn producer_factory(
    shared_state: Rc<Cell<State>>,
    quantity: u8,
    count_key: StateKey<usize>,
    passivated_list: StateKey<PassivatedList>,
    simulation: &mut Simulation<Effect>,
) {
    let mut state = shared_state.take();
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
        simulation.schedule_event(0.0, producer, Effect::TimeOut(0.));
    }
    shared_state.set(state);
}

pub fn consumer_factory(
    shared_state: Rc<Cell<State>>,
    quantity: u8,
    count_key: StateKey<usize>,
    passivated_list: StateKey<PassivatedList>,
    simulation: &mut Simulation<Effect>,
) {
    let mut state = shared_state.take();
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
        simulation.schedule_event(0.0, consumer, Effect::TimeOut(0.));
    }
    shared_state.set(state);
}
