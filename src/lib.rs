#[cfg(test)]
#[macro_use]
extern crate hamcrest;

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    fn should_die(number_of_neighbours: u32) -> bool {
        true
    }

    #[test]
    fn cell_must_die_if_zero_neighbours() {
        assert_that(should_die(0), is(true));
    }
}
