#[cfg(test)]
#[macro_use]
extern crate hamcrest;

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    fn should_die(number_of_neighbours: u32) -> bool {
        match number_of_neighbours {
            2 | 3 => false,
            _ => true,
        }
    }

    #[test]
    fn cell_must_die_if_zero_neighbours() {
        assert_that(should_die(0), is(true));
    }

    #[test]
    fn cell_must_die_if_one_neighbours() {
        assert_that(should_die(1), is(true));
    }

    #[test]
    fn cell_must_not_die_if_two_neighbours() {
        assert_that(should_die(2), is(false));
    }

    #[test]
    fn cell_must_not_die_if_three_neighbours() {
        assert_that(should_die(3), is(false));
    }

    #[test]
    fn cell_must_die_if_four_neighbours() {
        assert_that(should_die(4), is(true));
    }
}
