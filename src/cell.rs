use std::fmt;
use place::Place;

/// This struct represents a living cell.
#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    position: Place,
    is_dead: bool,
}

impl Cell {
    /// Create a new cell a given position.
    pub fn new(position: Place) -> Cell {
        Cell { position, is_dead: false }
    }

    /// Get the position of the cell.
    pub fn get_position(&self) -> &Place {
        &self.position
    }

    fn kill(&self) -> Cell {
        Cell { position: self.position.clone(), is_dead: true }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_dead {
            write!(f, "☼")
        } else {
            write!(f, "☀")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn format_dead() {
        let sut = Cell::new(Place::new(1, 1)).kill();

        assert_that!(format!("{}", sut), is(equal_to(String::from("☼"))));
    }

    #[test]
    fn format_alive() {
        let sut = Cell::new(Place::new(1, 1));

        assert_that!(format!("{}", sut), is(equal_to(String::from("☀"))));
    }
}