use std::fmt;

/// This struct describes the status of a population.
#[derive(Debug, PartialEq, Clone)]
pub struct Status {
    /// The iteration of in which the population was.
    iteration: usize,
    /// Number of cells the population has.
    cells: usize,
    /// How many cells were born in comparison to the previous iteration.
    born: usize,
    /// How many cells were died in comparison to the previous iteration.
    died: usize,
}

impl Status {
    /// Creates a new status.
    pub fn new(iteration: usize, cells: usize, born: usize, died: usize) -> Status {
        Status { iteration, cells, born, died }
    }

    /// Get the iteration count.
    pub fn get_iteration(&self) -> usize {
        self.iteration
    }

    /// Get the number of living cells of this iteration.
    pub fn get_cells(&self) -> usize {
        self.cells
    }

    /// Get the number of born cells in this iteration.
    pub fn get_born(&self) -> usize {
        self.born
    }

    /// Get the number of died cells in this iteration.
    pub fn get_died(&self) -> usize {
        self.died
    }

}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Iteration: {}, Cells: {}, Born: {}, Died: {}",
            self.iteration,
            self.cells,
            self.born,
            self.died)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn stringify() {
        let status = Status::new(42, 23, 5, 3);

        assert_that!(
            format!("{}", status),
            is(equal_to(String::from("Iteration: 42, Cells: 23, Born: 5, Died: 3"))));
    }
}