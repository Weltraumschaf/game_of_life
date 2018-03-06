use place::Place;

/// This struct represents a living cell.
#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    position: Place,
}

impl Cell {
    /// Create a new cell a given position.
    pub fn new(position: Place) -> Cell {
        Cell { position }
    }

    /// Get the position of the cell.
    pub fn get_position(&self) -> &Place {
        &self.position
    }
}
