#[cfg(test)]
#[macro_use]
extern crate hamcrest;
extern crate clap;
extern crate rand;

mod cell;
pub mod config;
mod dimension;
mod place;
mod population;
mod screen;
mod status;

use std::thread;
use rand::Rng;

use place::Place;
use cell::Cell;
use config::Config;
use population::Population;
use screen::{clear, print_header};

pub fn run_game(config: Config) {
    let mut population = create_initial_population(&config);
    let mut previous_status = population.get_status();

    loop {
        clear();
        print_header();
        println!("{}", &config);
        println!("{}", population.get_status());
        println!();
        print!("{}", population);
        population = population.next_generation();

        if previous_status.is_population_unchanged(population.get_status()) {
            println!("Population is stuck! No more evolution...");
            break;
        }

        previous_status = population.get_status();
        thread::sleep(config.get_sleep());
    }
}
/// Generate a random population of cells.
fn create_initial_population(config: &Config) -> Population {
    let mut cells: Vec<Cell> = Vec::new();
    let mut rng = rand::thread_rng();

    for y in 0..config.get_height() {
        for x in 0..config.get_width() {
            if rng.next_u32() % config.get_ratio() == 0 {
                cells.push(Cell::new(Place::new(x, y)));
            }
        }
    }

    Population::new(
        config.get_width(),
        config.get_height(),
        cells)
}
