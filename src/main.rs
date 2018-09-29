#![allow(dead_code)]
extern crate rand;
extern crate regex;
#[macro_use] extern crate lazy_static;

mod cards;
mod game;
mod parser;
mod commands;

fn main() {
    let mut rng = rand::thread_rng();
    let mut state = game::State::new(100, &mut rng);
    game::play(&mut state);
}