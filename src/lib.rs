#[cfg(test)]
#[macro_use]
extern crate hamcrest;
extern crate clap;
extern crate rand;

mod cell;
pub mod config;
mod dimension;
mod place;
pub mod population;
pub mod screen;
pub mod status;

use rand::Rng;
use place::Place;
use cell::Cell;
use config::Config;
use population::Population;

/// Generate a random population of cells.
pub fn create_initial_population(config: &Config) -> Population {
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
