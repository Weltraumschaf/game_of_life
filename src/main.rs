#[cfg(test)]
#[macro_use]
extern crate hamcrest;
extern crate game_of_live;

use game_of_live::Population;
use game_of_live::Cell;
use game_of_live::Place;

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

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;
}
