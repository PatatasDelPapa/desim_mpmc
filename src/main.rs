#![feature(generators, generator_trait)]

use std::{cell::Cell, rc::Rc};

use desim::{Simulation, Effect};
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
    state.insert(0);

    let producer_key_1 = state.insert(usize::MAX);
    let consumer_key_1 = state.insert(usize::MAX);

    shared_state.set(state);
    simulation
}