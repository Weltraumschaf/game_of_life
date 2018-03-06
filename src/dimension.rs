use std::fmt;

/// Describes the dimension of a population.
#[derive(Debug, PartialEq, Clone)]
pub struct Dimension {
    width: usize,
    height: usize,
}

impl Dimension {
    /// Create new dimension.
    pub fn new(width: usize, height: usize) -> Dimension {
        Dimension { width, height }
    }

    /// Get width of dimension.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Get height of dimension.
    pub fn get_height(&self) -> usize {
        self.height
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}x{})", self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn format_place() {
        assert_that!(format!("{}", Dimension::new(23, 42)), is(equal_to(String::from("(23x42)"))));
    }
}