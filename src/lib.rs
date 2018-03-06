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

/// This function calculates the distance vector of two places.
///
/// ```text
///                      ->   / b1 - a1 \
/// A(a1, a2), B(b1, b2) AB = |         |
///                           \ b2 - a2 /
/// ```
fn connection_vector(a: &Place, b: &Place) -> (isize, isize) {
    (
        b.get_x() as isize - a.get_x() as isize,
        b.get_y() as isize - a.get_y() as isize
    )
}

/// This function calculates the distance of two places.
fn distance(a: &Place, b: &Place) -> f64 {
    let (a, b) = connection_vector(a, b);
    let powered_sum = a.pow(2) + b.pow(2);

    (powered_sum as f64).sqrt()
}

/// This struct describes the status of a cell population.
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

    fn get_iteration(&self) -> usize {
        self.iteration
    }

    fn get_cells(&self) -> usize {
        self.cells
    }

    fn get_born(&self) -> usize {
        self.born
    }

    fn get_died(&self) -> usize {
        self.died
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stringify())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Population {
    iteration: usize,
    width: usize,
    height: usize,
    cells: Vec<Cell>
}

impl Population {
    pub fn new(width: usize, height: usize, cells: Vec<Cell>) -> Population {
        Population {
            iteration: 0,
            width,
            height,
            cells,
        }
    }

    fn get_cells(&self) -> Vec<Cell> {
        self.cells.clone()
    }

    fn next_generation(&self) -> Population {
        for y in 0..self.height {
            for x in 0..self.width {
                let current = Place::new(x, y);
                let number_of_neighbours = 0;//count_neighbours(&self.cells, &current);

                match self.get_cell(&current) {
                    Some(_) => {},
                    None => {},
                }
            }
        }

        Population {
            iteration: self.iteration + 1,
            width: self.width,
            height: self.height,
            cells: self.cells.clone(),
        }
    }

    pub fn get_status(&self) -> Status {
        Status::new(self.iteration, self.cells.len(), 0, 0)
    }

    fn stringify(&self) -> String {
        let mut buf = String::new();
        buf.push_str(self.get_status().stringify().as_str());
        buf.push('\n');

        for y in 0..self.height {
            for x in 0..self.width {
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
        it.find(|cell| cell.position == *position)
    }
}

impl fmt::Display for Population {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stringify())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    position: Place,
}

impl Cell {
    pub fn new(position: Place) -> Cell {
        Cell { position }
    }

    fn get_position(&self) -> &Place {
        &self.position
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Place {
    x: usize,
    y: usize,
}

impl Place {
    pub fn new(x: usize, y: usize) -> Place {
        Place { x, y }
    }

    fn get_x(&self) -> usize {
        self.x
    }

    fn get_y(&self) -> usize {
        self.y
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
    fn count_neighbours_empty_vector() {
        assert_that!(count_neighbours(&Vec::new(), &Place::new(1, 1)), is(equal_to(0)));
    }

    #[test]
    fn test_connection_vector_for_zero_points() {
        assert_that!(connection_vector(&Place::new(0, 0), &Place::new(0, 0)), is(equal_to((0, 0))));
    }

    #[test]
    fn test_connection_vector_for_same_points() {
        assert_that!(connection_vector(&Place::new(42, 23), &Place::new(42, 23)), is(equal_to((0, 0))));
    }

    #[test]
    fn test_connection_vector_for_first_point_is_zero() {
        assert_that!(connection_vector(&Place::new(0, 0), &Place::new(42, 23)), is(equal_to((42, 23))));
    }

    #[test]
    fn test_connection_vector_for_second_point_is_zero() {
        assert_that!(connection_vector(&Place::new(42, 23), &Place::new(0, 0)), is(equal_to((-42, -23))));
    }

    #[test]
    fn test_connection_vector_for_two_different_places() {
        assert_that!(connection_vector(&Place::new(5, 10), &Place::new(7, 8)), is(equal_to((2, -2))));
    }

    #[test]
    fn distance_of_zero_places() {
        assert_that!(distance(&Place::new(0, 0), &Place::new(0 ,0)), is(equal_to(0.0)));
    }

    #[test]
    fn distance_of_same_places() {
        assert_that!(distance(&Place::new(3, 3), &Place::new(3 ,3)), is(equal_to(0.0)));
    }

    #[test]
    fn distance_of_two_neighbours() {
        assert_that!(distance(&Place::new(5, 5), &Place::new(4, 4)), is(close_to(1.4142, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(5, 4)), is(close_to(1.0, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(6, 4)), is(close_to(1.4142, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(4, 5)), is(close_to(1.0, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(6, 5)), is(close_to(1.0, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(4, 6)), is(close_to(1.4142, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(5, 6)), is(close_to(1.0, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(6, 6)), is(close_to(1.4142, 0.0001)));
    }

    #[test]
    fn distance_of_two_not_neighbours() {
        assert_that!(distance(&Place::new(5, 5), &Place::new(3, 3)), is(close_to(2.8284, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(4, 3)), is(close_to(2.2360, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(5, 3)), is(close_to(2.0, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(6, 3)), is(close_to(2.2360, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(7, 3)), is(close_to(2.8284, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(3, 4)), is(close_to(2.2360, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(7, 4)), is(close_to(2.2360, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(3, 5)), is(close_to(2.0, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(7, 5)), is(close_to(2.0, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(3, 6)), is(close_to(2.2360, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(7, 7)), is(close_to(2.8284, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(3, 7)), is(close_to(2.8284, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(4, 7)), is(close_to(2.2360, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(5, 7)), is(close_to(2.0, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(6, 7)), is(close_to(2.2360, 0.0001)));
        assert_that!(distance(&Place::new(5, 5), &Place::new(7, 7)), is(close_to(2.8284, 0.0001)));
    }

    /// This function counts the number of neighbours (living cells) for a given place.
    fn count_neighbours(cells: &Vec<Cell>, position: &Place) -> usize {
        let mut neighbours = 0;

        for cell in cells {
            let distance = distance(cell.get_position(), position);

            if distance <= 1.0 {
                neighbours += 1;
            }
        }

        neighbours
    }

    #[test]
    #[ignore]
    fn count_neighbours_surrounded_by_eight_neighbours() {
        // (4,4) (5,4) (6,4)
        // (4,5) [5,5] (6,5)
        // (4,6) (5,6) (6,6)
        let cells = vec![
            Cell::new(Place::new(4, 4)),
            Cell::new(Place::new(5, 4)),
            Cell::new(Place::new(6, 4)),
            Cell::new(Place::new(4, 5)),
            Cell::new(Place::new(6, 5)),
            Cell::new(Place::new(4, 6)),
            Cell::new(Place::new(5, 6)),
            Cell::new(Place::new(6, 6))
        ];
        assert_that!(count_neighbours(&cells, &Place::new(5, 5)), is(equal_to(8)));
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
    #[ignore]
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
}
