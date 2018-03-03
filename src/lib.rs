#[cfg(test)]
#[macro_use]
extern crate hamcrest;

use std::fmt;

/// A cell should die if it has less than two or more than three neighbours.
fn should_die(number_of_neighbours: u32) -> bool {
    match number_of_neighbours {
        2 | 3 => false,
        _ => true,
    }
}

/// At an empty place a new cell should spawn, if this cell has exactly three living cells as
/// neighbour.
fn should_spawn(number_of_neighbours: u32) -> bool {
    number_of_neighbours == 3
}

#[derive(Debug, PartialEq, Clone)]
pub struct Status {
    iteration: u32,
    cells: u32,
    born: u32,
    died: u32,
}

impl Status {
    pub fn new(iteration: u32, cells: u32, born: u32, died: u32) -> Status {
        Status { iteration, cells, born, died }
    }

    pub fn stringify(&self) -> String {
        format!(
            "Iteration: {}, Cells: {}, Born: {}, Died: {}",
            self.iteration,
            self.cells,
            self.born,
            self.died)
    }

    pub fn get_iteration(&self) -> u32 {
        self.iteration
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stringify())
    }
}

#[derive(Debug, PartialEq)]
pub struct Population {
    status: Status,
}

impl Population {
    pub fn new() -> Population {
        Population { status: Status::new(0, 0, 0, 0) }
    }

    fn next_generation(&self) -> Population {
        Population {
            status: Status::new(self.status.get_iteration() + 1, 0, 0, 0)
        }
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn cell_must_die_if_zero_neighbours() {
        assert_that!(should_die(0), is(true));
    }

    #[test]
    fn cell_must_die_if_one_neighbours() {
        assert_that!(should_die(1), is(true));
    }

    #[test]
    fn cell_must_not_die_if_two_neighbours() {
        assert_that!(should_die(2), is(false));
    }

    #[test]
    fn cell_must_not_die_if_three_neighbours() {
        assert_that!(should_die(3), is(false));
    }

    #[test]
    fn cell_must_die_if_four_neighbours() {
        assert_that!(should_die(4), is(true));
    }

    #[test]
    fn cell_must_die_if_five_neighbours() {
        assert_that!(should_die(5), is(true));
    }

    #[test]
    fn cell_must_die_if_six_neighbours() {
        assert_that!(should_die(6), is(true));
    }

    #[test]
    fn cell_must_die_if_seven_neighbours() {
        assert_that!(should_die(7), is(true));
    }

    #[test]
    fn cell_must_die_if_eight_neighbours() {
        assert_that!(should_die(8), is(true));
    }

    #[test]
    fn should_not_spawn_new_cell_if_zero_neighbours() {
        assert_that!(should_spawn(0), is(false));
    }

    #[test]
    fn should_not_spawn_new_cell_if_one_neighbours() {
        assert_that!(should_spawn(1), is(false));
    }

    #[test]
    fn should_not_spawn_new_cell_if_two_neighbours() {
        assert_that!(should_spawn(2), is(false));
    }

    #[test]
    fn should_spawn_new_cell_if_three_neighbours() {
        assert_that!(should_spawn(3), is(true));
    }

    #[test]
    fn should_not_spawn_new_cell_if_four_neighbours() {
        assert_that!(should_spawn(4), is(false));
    }

    #[test]
    fn should_not_spawn_new_cell_if_five_neighbours() {
        assert_that!(should_spawn(5), is(false));
    }

    #[test]
    fn should_not_spawn_new_cell_if_six_neighbours() {
        assert_that!(should_spawn(6), is(false));
    }

    #[test]
    fn should_not_spawn_new_cell_if_seven_neighbours() {
        assert_that!(should_spawn(7), is(false));
    }

    #[test]
    fn should_not_spawn_new_cell_if_eight_neighbours() {
        assert_that!(should_spawn(8), is(false));
    }

    #[test]
    fn format_game_status() {
        let status = Status::new(42, 23, 5, 3);

        assert_that!(
            status.stringify(),
            is(equal_to(String::from("Iteration: 42, Cells: 23, Born: 5, Died: 3"))));
    }

    #[test]
    fn new_population_has_initial_status() {
        let initial = Population::new();

        assert_that!(initial.get_status(), is(equal_to(Status::new(0, 0, 0, 0))));
    }

    #[test]
    fn generate_next_population() {
        let initial = Population::new();

        let next = initial.next_generation();

        assert_that!(initial.get_status().get_iteration(), is(equal_to(0)));
        assert_that!(next.get_status().get_iteration(), is(equal_to(1)));
    }
}
