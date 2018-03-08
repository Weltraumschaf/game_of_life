extern crate game_of_live;

use game_of_live::population::Population;
use game_of_live::cell::Cell;
use game_of_live::place::Place;

/// The main entry point of the binary.
///
/// This create follows the paradigm of an executable library: This means that the whole application
/// is in the library part of the crate (and so can be used as a dependency). The main module only
/// provides the main function with simple code which delegates to the library part.
fn main() {
    println!("Game of Live");
    println!("============");
    println!();

    let cells: Vec<Cell> = vec!(
        Cell::new(Place::new(0, 0)),
        Cell::new(Place::new(9, 0)),
        Cell::new(Place::new(0, 1)),
        Cell::new(Place::new(2, 1)),
        Cell::new(Place::new(7, 1)),
        Cell::new(Place::new(9, 1)),
        Cell::new(Place::new(0, 2)),
        Cell::new(Place::new(3, 2)),
        Cell::new(Place::new(6, 2)),
        Cell::new(Place::new(9, 2)),
        Cell::new(Place::new(0, 3)),
        Cell::new(Place::new(4, 3)),
        Cell::new(Place::new(5, 3)),
        Cell::new(Place::new(9, 3)),
        Cell::new(Place::new(0, 4)),
        Cell::new(Place::new(9, 4))
    );
    let initial = Population::new(10, 5, cells);
    println!("{}", initial);
}
