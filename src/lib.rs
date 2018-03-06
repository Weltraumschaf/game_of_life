#[cfg(test)]
#[macro_use]
extern crate hamcrest;

pub mod cell;
pub mod place;
pub mod population;
mod status;

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;


}
