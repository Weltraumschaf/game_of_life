#[cfg(test)]
#[macro_use]
extern crate hamcrest;
extern crate game_of_live;

use game_of_live::Population;

fn main() {
    println!("Game of Live");
    println!("============");
    println!();

    let initial = Population::new();
    println!("{}", initial.get_status());
}

struct Cell {
    position: Place,
}

struct Place {
    x: u32,
    y: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;
}
