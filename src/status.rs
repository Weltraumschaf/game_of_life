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
    pub fn new(iteration: usize, cells: usize, born: usize, died: usize) -> Status {
        Status { iteration, cells, born, died }
    }

    fn stringify(&self) -> String {
        format!(
            "Iteration: {}, Cells: {}, Born: {}, Died: {}",
            self.iteration,
            self.cells,
            self.born,
            self.died)
    }

    pub fn get_iteration(&self) -> usize {
        self.iteration
    }

    pub fn get_cells(&self) -> usize {
        self.cells
    }

    pub fn get_born(&self) -> usize {
        self.born
    }

    pub fn get_died(&self) -> usize {
        self.died
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stringify())
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
            status.stringify(),
            is(equal_to(String::from("Iteration: 42, Cells: 23, Born: 5, Died: 3"))));
    }

}