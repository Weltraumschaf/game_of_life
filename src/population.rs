use cell::Cell;
use place::Place;
use place::distance;
use std::fmt;
use status::Status;

/// Describes the dimension of a population.
#[derive(Debug, PartialEq, Clone)]
struct Dimension {
    width: usize,
    height: usize,
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}x{})", self.width, self.height)
    }
}

/// This struct describes a population of cells.
#[derive(Debug, PartialEq, Clone)]
pub struct Population {
    /// The status of this population.
    status: Status,
    /// The dimension of the population.
    size: Dimension,
    /// The living cells of this population.
    cells: Vec<Cell>,
}

impl Population {
    /// Create a new population.
    pub fn new(width: usize, height: usize, cells: Vec<Cell>) -> Population {
        Population {
            status: Status::new(0, cells.len(), 0, 0),
            size: Dimension { width, height },
            cells,
        }
    }

    fn get_status(&self) -> Status {
        self.status.clone()
    }

    fn get_cells(&self) -> Vec<Cell> {
        self.cells.clone()
    }

    fn next_generation(&self) -> Population {
        let mut survived: Vec<Cell> = Vec::new();
        let mut born = 0;
        let mut died = 0;

        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let current = Place::new(x, y);
                let number_of_neighbours = count_neighbours(&self.cells, &current);

                match self.get_cell(&current) {
                    Some(cell) => {
                        if should_die(number_of_neighbours) {
                            died += 1;
                        } else {
                            survived.push(cell);
                        }
                    },
                    None => {
                        if should_spawn(number_of_neighbours) {
                            born += 1;
                            survived.push(Cell::new(current));
                        }
                    },
                }
            }
        }

        let iteration = self.get_status().get_iteration() + 1;

        Population {
            status: Status::new(iteration, survived.len(), born, died),
            size: self.size.clone(),
            cells: survived,
        }
    }

    fn stringify(&self) -> String {
        let mut buf = String::new();
        buf.push_str(&format!("{}", self.get_status()));
        buf.push('\n');

        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let current = Place::new(x, y);

                match self.get_cell(&current) {
                    Some(_) => buf.push('O'),
                    None => buf.push(' '),
                }
            }

            buf.push('\n');
        }

        buf
    }

    fn get_cell(&self, position: &Place) -> Option<Cell> {
        let mut it = self.get_cells().into_iter();
        it.find(|cell| cell.get_position() == position)
    }
}

impl fmt::Display for Population {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stringify())
    }
}

/// A cell should die if it has less than two or more than three neighbours.
fn should_die(number_of_neighbours: usize) -> bool {
    match number_of_neighbours {
        2 | 3 => false,
        _ => true,
    }
}

/// At an empty place a new cell should spawn, if this cell has exactly three living cells as
/// neighbour.
fn should_spawn(number_of_neighbours: usize) -> bool {
    number_of_neighbours == 3
}

/// This function counts the number of neighbours (living cells) for a given place.
fn count_neighbours(cells: &Vec<Cell>, position: &Place) -> usize {
    let mut neighbours = 0;

    for cell in cells {
        let distance = distance(cell.get_position(), position);

        if distance > 0.0 && distance < 2.0 {
            neighbours += 1;
        }
    }

    neighbours
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn new_population_has_initial_status() {
        let initial = Population::new(5, 5, Vec::new());

        assert_that!(initial.get_status(), is(equal_to(Status::new(0, 0, 0, 0))));
    }

    #[test]
    fn generate_next_population_from_empty_population() {
        let initial = Population::new(5, 5, Vec::new());

        let next = initial.next_generation();

        assert_that!(initial.get_status().get_iteration(), is(equal_to(0)));
        assert_that!(next.get_status().get_iteration(), is(equal_to(1)));
        assert_that!(next.get_status().get_cells(), is(equal_to(0)));
    }

    #[test]
    fn generate_next_population_single_cell_must_die() {
        let cells: Vec<Cell> = vec![
            Cell::new(Place::new(9, 4))
        ];
        let sut = Population::new(10, 5, cells);

        let next = sut.next_generation().get_status();

        assert_that!(next.get_cells(), is(equal_to(0)));
        assert_that!(next.get_died(), is(equal_to(1)));
    }

    #[test]
    #[ignore]
    fn generate_next_population_cell_with_one_neighbour_must_die() {}

    #[test]
    #[ignore]
    fn generate_next_population_cell_with_two_neighbours_survives() {}

    #[test]
    #[ignore]
    fn generate_next_population_cell_with_three_neighbours_survive() {}

    #[test]
    #[ignore]
    fn generate_next_population_cell_with_four_neighbours_must_die() {}

    #[test]
    #[ignore]
    fn generate_next_population_cell_with_five_neighbours_must_die() {}

    #[test]
    #[ignore]
    fn generate_next_population_cell_with_six_neighbours_must_die() {}

    #[test]
    #[ignore]
    fn generate_next_population_cell_with_seven_neighbours_must_die() {}

    #[test]
    #[ignore]
    fn generate_next_population_cell_with_eight_neighbours_must_die() {}

    #[test]
    #[ignore]
    fn generate_next_population_new_cell_will_be_born_on_three_neighbours_at_empty_place() {}

    #[test]
    fn format_empty_population() {
        let sut = Population::new(10, 5, Vec::new());
        let expected = "Iteration: 0, Cells: 0, Born: 0, Died: 0\n          \n          \n          \n          \n          \n";

        assert_that!(sut.stringify(), is(equal_to(String::from(expected))));
    }

    #[test]
    fn format_some_population() {
        let cells: Vec<Cell> = vec![
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
        ];
        let sut = Population::new(10, 5, cells);
        let expected = r#"Iteration: 0, Cells: 16, Born: 0, Died: 0
O        O
O O    O O
O  O  O  O
O   OO   O
O        O
"#;

        assert_that!(sut.stringify(), is(equal_to(String::from(expected))));
    }

    #[test]
    fn get_cell_not_found() {
        let sut = Population::new(5, 5, Vec::new());

        assert_that!(sut.get_cell(&Place::new(1, 1)), is(equal_to(None)));
    }

    #[test]
    fn get_cell_found() {
        let sut = Population::new(
            5,
            5,
            vec![Cell::new(Place::new(1, 1))]);

        assert_that!(sut.get_cell(&Place::new(1, 1)), is(equal_to(Some(Cell::new(Place::new(1, 1))))));
    }

    #[test]
    fn get_number_of_cells_in_status() {
        let cells: Vec<Cell> = vec![
            Cell::new(Place::new(0, 0)),
            Cell::new(Place::new(9, 0)),
            Cell::new(Place::new(0, 1)),
            Cell::new(Place::new(2, 1)),
            Cell::new(Place::new(7, 1))
        ];

        let sut = Population::new(10, 5, cells);

        assert_that!(sut.get_status().get_cells(), is(equal_to(5)));
    }

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
    fn count_neighbours_empty_vector() {
        assert_that!(count_neighbours(&Vec::new(), &Place::new(1, 1)), is(equal_to(0)));
    }

    #[test]
    fn count_neighbours_surrounded_by_zero_neighbours() {
        let cells = vec![
            Cell::new(Place::new(5, 3)),
            Cell::new(Place::new(5, 5)),
            Cell::new(Place::new(6, 7))
        ];

        assert_that!(count_neighbours(&cells, &Place::new(5, 5)), is(equal_to(0)));
    }

    #[test]
    fn count_neighbours_surrounded_by_two_neighbours() {
        let cells = vec![
            Cell::new(Place::new(5, 4)),
            Cell::new(Place::new(5, 5)),
            Cell::new(Place::new(6, 6))
        ];

        assert_that!(count_neighbours(&cells, &Place::new(5, 5)), is(equal_to(2)));
    }

    #[test]
    fn count_neighbours_surrounded_by_eight_neighbours() {
        // (4,4) (5,4) (6,4)
        // (4,5) [5,5] (6,5)
        // (4,6) (5,6) (6,6)
        let cells = vec![
            Cell::new(Place::new(4, 4)),
            Cell::new(Place::new(5, 4)),
            Cell::new(Place::new(6, 4)),
            Cell::new(Place::new(4, 5)),
            Cell::new(Place::new(5, 5)),
            Cell::new(Place::new(6, 5)),
            Cell::new(Place::new(4, 6)),
            Cell::new(Place::new(5, 6)),
            Cell::new(Place::new(6, 6))
        ];

        assert_that!(count_neighbours(&cells, &Place::new(5, 5)), is(equal_to(8)));
    }
}