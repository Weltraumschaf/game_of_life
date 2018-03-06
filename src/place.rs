use std::fmt;

/// This struct represents a place.
#[derive(Debug, PartialEq, Clone)]
pub struct Place {
    /// X-axis position of place beginning a zero.
    x: usize,
    /// Y-axis position of place beginning a zero.
    y: usize,
}

impl Place {
    /// Create a new place.
    pub fn new(x: usize, y: usize) -> Place {
        Place { x, y }
    }

    /// Get the x-position of this place.
    pub fn get_x(&self) -> usize {
        self.x
    }

    /// Get the y-position of this place.
    pub fn get_y(&self) -> usize {
        self.y
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.get_x(), self.get_y())
    }
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
///
/// ```text
///
/// |-->|    +-------------------
/// | a | = \|  ax * ax + ay * ay
/// |   |    V
/// ```
pub fn distance(a: &Place, b: &Place) -> f64 {
    let (a, b) = connection_vector(a, b);
    let powered_sum = a.pow(2) + b.pow(2);

    (powered_sum as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn format_place() {
        assert_that!(format!("{}", Place::new(23, 42)), is(equal_to(String::from("(23, 42)"))));
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
}