extern crate game_of_life;
extern crate clap;

use std::thread;
use clap::{Arg, App};
use game_of_life::cell::Cell;
use game_of_life::config::create_config;
use game_of_life::place::Place;
use game_of_life::population::Population;
use game_of_life::screen::*;

/// The main entry point of the binary.
///
/// This create follows the paradigm of an executable library: This means that the whole application
/// is in the library part of the crate (and so can be used as a dependency). The main module only
/// provides the main function with simple code which delegates to the library part.
fn main() {
    let matches = App::new("Game of Life")
        .version("1.0.0")
        .author("Sven Strittmatter <ich@weltraumschaf.de>")
        .about("This is a Game of Life implementation.")
        .arg(Arg::with_name("width")
            .long("width")
            .value_name("WIDTH")
            .help("Sets width of the population space. Default is 20.")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .long("height")
            .value_name("HEIGHT")
            .help("Sets height of the population space. Default is 20.")
            .takes_value(true))
        .arg(Arg::with_name("sleep")
            .long("sleep")
            .value_name("SLEEP")
            .help("Sets sleep time in seconds between the population iterations. Default is 1.")
            .takes_value(true))
        .get_matches();
    let config = create_config(&matches);

    let mut population = Population::new(
        config.get_width(),
        config.get_height(),
        create_initial_cells());

    loop {
        clear();
        print_header();
        println!("{}", population);
        population = population.next_generation();
        thread::sleep(config.get_sleep());
    }
}

fn create_initial_cells() -> Vec<Cell> {
    vec![
        Cell::new(Place::new(13, 9)),
        Cell::new(Place::new(12, 10)),
        Cell::new(Place::new(13, 10)),
        Cell::new(Place::new(14, 10)),
        Cell::new(Place::new(13, 11))
    ]
}
