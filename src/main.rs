extern crate game_of_life;
extern crate clap;

use std::{thread, time};
use clap::{Arg, App, ArgMatches};
use game_of_life::population::Population;
use game_of_life::cell::Cell;
use game_of_life::place::Place;

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
    let (width, height, sleep) = get_config(&matches);

    let mut population = Population::new(width, height, create_initial_cells());

    loop {
        clear_screen();
        print_header();
        println!("{}", population);
        population = population.next_generation();
        wait(sleep);
    }
}

fn get_config(matches: &ArgMatches) -> (usize, usize, u64) {
    let width = matches.value_of("width").unwrap_or("20");
    let width = width.parse::<usize>().expect("Not negative number expected as width!");

    let height = matches.value_of("height").unwrap_or("20");
    let height = height.parse::<usize>().expect("Not negative number expected as height!");

    let sleep = matches.value_of("sleep").unwrap_or("1");
    let sleep = sleep.parse::<u64>().expect("Not negative number expected as sleep!");

    (width, height, sleep)
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

fn print_header() {
    println!("Game of Live");
    println!("============");
    println!();
}

fn wait(sleep: u64) {
    let pause = time::Duration::from_secs(sleep);
    thread::sleep(pause);
}

fn clear_screen() {
    print!("\x1b[2J\x1b[1;1H");
}