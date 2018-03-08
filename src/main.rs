extern crate game_of_life;

use game_of_life::population::Population;
use game_of_life::cell::Cell;
use game_of_life::place::Place;

/// The main entry point of the binary.
///
/// This create follows the paradigm of an executable library: This means that the whole application
/// is in the library part of the crate (and so can be used as a dependency). The main module only
/// provides the main function with simple code which delegates to the library part.
fn main() {
    println!("Game of Live");
    println!("============");
    println!();

    let cells: Vec<Cell> = vec![
        Cell::new(Place::new(5, 3)),
        Cell::new(Place::new(6, 3)),
        Cell::new(Place::new(7, 3))
    ];

    let initial = Population::new(10, 5, cells);
    let next = initial.next_generation();
    println!("{}", next);
}
