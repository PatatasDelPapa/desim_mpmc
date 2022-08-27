#![feature(generators, generator_trait)]
#![allow(unused)]

use desim_mpmc::simulation;

fn main() {
    simulation(10.0, 2, 3);
}
