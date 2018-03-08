#[cfg(test)]
#[macro_use]
extern crate hamcrest;
extern crate clap;

mod cell;
pub mod config;
mod dimension;
mod place;
pub mod population;
pub mod screen;
mod status;

use place::Place;
use cell::Cell;

pub fn create_initial_cells() -> Vec<Cell> {
    vec![
        Cell::new(Place::new(13, 9)),
        Cell::new(Place::new(12, 10)),
        Cell::new(Place::new(13, 10)),
        Cell::new(Place::new(14, 10)),
        Cell::new(Place::new(13, 11))
    ]
}
