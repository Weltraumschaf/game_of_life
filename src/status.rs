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

    /// This method increases the iteration property by one and returns a new status. The original
    /// status will be unchanged.
    pub fn inc_iteration(&self) -> Status {
        Status {
            iteration: self.get_iteration() + 1,
            cells: self.get_cells(),
            born: self.get_born(),
            died: self.get_died(),
        }
    }

    /// Get the number of living cells of this iteration.
    pub fn get_cells(&self) -> usize {
        self.cells
    }

    /// This method increases the cells property by one and returns a new status. The original
    /// status will be unchanged.
    pub fn inc_cells(&self) -> Status {
        Status {
            iteration: self.get_iteration(),
            cells: self.get_cells() + 1,
            born: self.get_born(),
            died: self.get_died(),
        }
    }
    /// Get the number of born cells in this iteration.
    pub fn get_born(&self) -> usize {
        self.born
    }

    /// This method increases the born property the cells property by one and returns a new status.
    /// The original status will be unchanged.
    pub fn inc_born(&self) -> Status {
        Status {
            iteration: self.get_iteration(),
            cells: self.get_cells() + 1,
            born: self.get_born() + 1,
            died: self.get_died(),
        }
    }

    /// Get the number of died cells in this iteration.
    pub fn get_died(&self) -> usize {
        self.died
    }

    /// This method increases the died property by one and decreases the cells property by one and
    /// returns a new status. The original status will be unchanged.
    pub fn inc_died(&self) -> Status {
        Status {
            iteration: self.get_iteration(),
            cells: self.get_cells() - 1,
            born: self.get_born(),
            died: self.get_died() + 1,
        }
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
    fn inc_iteration() {
        let sut = Status::new(23, 0, 0, 0).inc_iteration();

        assert_that!(sut.get_iteration(), is(equal_to(24)));
    }

    #[test]
    fn inc_cells() {
        let sut = Status::new(0, 23, 0, 0).inc_cells();

        assert_that!(sut.get_cells(), is(equal_to(24)));
    }

    #[test]
    fn inc_born() {
        let sut = Status::new(0, 10, 5, 0).inc_born();

        assert_that!(sut.get_cells(), is(equal_to(11)));
        assert_that!(sut.get_born(), is(equal_to(6)));
    }

    #[test]
    fn inc_died() {
        let sut = Status::new(0, 10, 0, 5).inc_died();

        assert_that!(sut.get_cells(), is(equal_to(9)));
        assert_that!(sut.get_died(), is(equal_to(6)));
    }

    #[test]
    fn fmt() {
        let status = Status::new(42, 23, 5, 3);

        assert_that!(
            format!("{}", status),
            is(equal_to(String::from("Iteration: 42, Cells: 23, Born: 5, Died: 3"))));
    }
}