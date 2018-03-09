use std::fmt;
use cell::Cell;
use dimension::Dimension;
use place::Place;
use place::distance;
use status::Status;

/// This struct describes a population of cells.
#[derive(PartialEq, Clone)]
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
            size: Dimension::new(width, height),
            cells,
        }
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    fn get_cells(&self) -> Vec<Cell> {
        self.cells.clone()
    }

    /// Generates the next evolution iteration of this population.
    pub fn next_generation(&self) -> Population {
        let (next, survived) = self.visit_all_places();

        if next.get_cells() != survived.len() {
            panic!(
                "This size of status cells and survived length must not differ: {} != {}!",
                next.get_cells(), survived.len()
            );
        }

        Population {
            status: next,
            size: self.size.clone(),
            cells: survived,
        }
    }

    fn visit_all_places(&self) -> (Status, Vec<Cell>) {
        let mut next = self.get_status().inc_iteration();
        let mut survived: Vec<Cell> = Vec::new();

        for y in 0..self.size.get_height() {
            for x in 0..self.size.get_width() {
                next = self.visit_place(Place::new(x, y), next, &mut survived);
            }
        }

        (next, survived)
    }

    fn visit_place(&self, current_place: Place, next: Status, survived: &mut Vec<Cell>) -> Status {
        let number_of_neighbours = count_neighbours(&self.cells, &current_place);

        match self.get_cell(&current_place) {
            Some(cell) => {
                if should_die(number_of_neighbours) {
                    return next.inc_died();
                } else {
                    survived.push(cell);
                }
            },
            None => {
                if should_spawn(number_of_neighbours) {
                    survived.push(Cell::new(current_place));
                    return next.inc_born();
                }
            },
        }

        next
    }

    fn get_cell(&self, position: &Place) -> Option<Cell> {
        let mut it = self.get_cells().into_iter();
        it.find(|cell| cell.get_position() == position)
    }

    fn has_cell(&self, position: &Place) -> bool {
        match self.get_cell(position) {
            Some(_) => true,
            None => false,
        }
    }
}

impl fmt::Display for Population {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&generate_line_for_population(self.size.get_width()));
        buf.push('\n');

        for y in 0..self.size.get_height() {
            buf.push('|');

            for x in 0..self.size.get_width() {
                let current = Place::new(x, y);

                match self.get_cell(&current) {
                    Some(_) => buf.push('☀'),
                    None => buf.push(' '),
                }
            }

            buf.push('|');
            buf.push('\n');
        }

        buf.push_str(&generate_line_for_population(self.size.get_width()));
        buf.push('\n');
        write!(f, "{}", buf)
    }
}

fn generate_line_for_population(width: usize) -> String {
    format!("+{}+", "-".repeat(width))
}

impl fmt::Debug for Population {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();

        for y in 0..self.size.get_height() {
            buf.push_str(&format!("{}", y));

            for x in 0..self.size.get_width() {
                let current = Place::new(x, y);

                match self.get_cell(&current) {
                    Some(_) => buf.push('O'),
                    None => buf.push(' '),
                }
            }

            buf.push('\n');
        }

        buf.push(' ');

        for x in 0..self.size.get_width() {
            buf.push_str(&format!("{}", x));
        }

        write!(f, "{}", buf)
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
    fn generate_line_for_population_zero_width() {
        assert_that!(generate_line_for_population(0), is(equal_to(String::from("++"))));
    }

    #[test]
    fn generate_line_for_population_some_width() {
        assert_that!(generate_line_for_population(5), is(equal_to(String::from("+-----+"))));
    }

    #[test]
    fn new_population_has_initial_status() {
        let initial = Population::new(5, 5, Vec::new());

        assert_that!(initial.get_status(), is(equal_to(Status::new(0, 0, 0, 0))));
    }

    #[test]
    fn generate_next_population_from_empty_population() {
        let sut = Population::new(5, 5, Vec::new());

        let next = sut.next_generation();

        assert_that!(sut.get_status().get_iteration(), is(equal_to(0)));
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
    fn generate_next_population_cell_with_one_neighbour_both_must_die() {
        let cells: Vec<Cell> = vec![
            Cell::new(Place::new(8, 4)),
            Cell::new(Place::new(9, 4))
        ];

        let sut = Population::new(10, 5, cells);
        let next = sut.next_generation().get_status();

        assert_that!(next.get_cells(), is(equal_to(0)));
        assert_that!(next.get_died(), is(equal_to(2)));
    }

    #[test]
    fn generate_next_population_cell_with_two_neighbours_survives() {
        let cells: Vec<Cell> = vec![
            Cell::new(Place::new(5, 3)),
            Cell::new(Place::new(6, 3)),
            Cell::new(Place::new(7, 3))
        ];

        let sut = Population::new(10, 5, cells);
        let next = sut.next_generation();

        assert_that!(next.get_status().get_cells(), is(equal_to(3)));
        assert_that!(next.get_status().get_died(), is(equal_to(2)));
        assert_that!(next.get_status().get_born(), is(equal_to(2)));
        assert_that!(next.has_cell(&Place::new(5, 3)), is(equal_to(false)));
        assert_that!(next.has_cell(&Place::new(7, 3)), is(equal_to(false)));
        assert_that!(next.has_cell(&Place::new(6, 2)), is(equal_to(true)));
        assert_that!(next.has_cell(&Place::new(6, 3)), is(equal_to(true)));
        assert_that!(next.has_cell(&Place::new(6, 4)), is(equal_to(true)));
    }

    #[test]
    fn generate_next_population_cell_with_three_neighbours_survive() {
        let cells: Vec<Cell> = vec![
            Cell::new(Place::new(2, 2)),
            Cell::new(Place::new(3, 2)),
            Cell::new(Place::new(4, 2)),
            Cell::new(Place::new(3, 3))
        ];

        let sut = Population::new(10, 5, cells);
        let next = sut.next_generation();

        assert_that!(next.get_status().get_cells(), is(equal_to(7)));
        assert_that!(next.get_status().get_died(), is(equal_to(0)));
        assert_that!(next.get_status().get_born(), is(equal_to(3)));

        assert_that!(next.has_cell(&Place::new(3, 1)), is(equal_to(true)));
        assert_that!(next.has_cell(&Place::new(2, 2)), is(equal_to(true)));
        assert_that!(next.has_cell(&Place::new(3, 2)), is(equal_to(true)));
        assert_that!(next.has_cell(&Place::new(4, 2)), is(equal_to(true)));
        assert_that!(next.has_cell(&Place::new(2, 3)), is(equal_to(true)));
        assert_that!(next.has_cell(&Place::new(3, 3)), is(equal_to(true)));
        assert_that!(next.has_cell(&Place::new(4, 3)), is(equal_to(true)));
    }

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
    fn format_display_empty_population() {
        let sut = Population::new(10, 5, Vec::new());
        let expected = r#"+----------+
|          |
|          |
|          |
|          |
|          |
+----------+
"#;

        assert_that!(format!("{}", sut), is(equal_to(String::from(expected))));
    }

    #[test]
    fn format_display_some_population() {
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
        let expected = r#"+----------+
|☀        ☀|
|☀ ☀    ☀ ☀|
|☀  ☀  ☀  ☀|
|☀   ☀☀   ☀|
|☀        ☀|
+----------+
"#;

        assert_that!(format!("{}", sut), is(equal_to(String::from(expected))));
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
    fn has_cell_not_found() {
        let sut = Population::new(5, 5, Vec::new());

        assert_that!(sut.has_cell(&Place::new(1, 1)), is(equal_to(false)));
    }

    #[test]
    fn has_cell_found() {
        let sut = Population::new(
            5,
            5,
            vec![Cell::new(Place::new(1, 1))]);

        assert_that!(sut.has_cell(&Place::new(1, 1)), is(equal_to(true)));
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