extern crate game_of_life;

use game_of_life::population::Population;
use game_of_life::cell::Cell;
use game_of_life::place::Place;
use std::{thread, time};

/// The main entry point of the binary.
///
/// This create follows the paradigm of an executable library: This means that the whole application
/// is in the library part of the crate (and so can be used as a dependency). The main module only
/// provides the main function with simple code which delegates to the library part.
fn main() {
    let cells: Vec<Cell> = vec![
        Cell::new(Place::new(13, 9)),
        Cell::new(Place::new(12, 10)),
        Cell::new(Place::new(13, 10)),
        Cell::new(Place::new(14, 10)),
        Cell::new(Place::new(13, 11))
    ];

    let mut population = Population::new(20, 20, cells);

    loop {
        clear_screen();
        print_header();
        println!("{}", population);
        population = population.next_generation();
        wait();
    }
}

fn print_header() {
    println!("Game of Live");
    println!("============");
    println!();
}

fn wait() {
    let pause = time::Duration::from_secs(1);
    thread::sleep(pause);
}

fn clear_screen() {
    print!("\x1b[2J\x1b[1;1H");
}