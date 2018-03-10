use clap::ArgMatches;
use std::fmt;
use std::time;
use std::time::Duration;

/// Default width of the game used if the CLI option is not given.
pub static DEFAULT_WIDTH: &'static str = "40";
/// Default height of the game used if the CLI option is not given.
pub static DEFAULT_HEIGHT: &'static str = "20";
/// Default sleep of the game used if the CLI option is not given.
pub static DEFAULT_SLEEP: &'static str = "1";
/// Default ratio of the game used if the CLI option is not given.
pub static DEFAULT_RATIO: &'static str = "4";

/// Creates a new config from the arguments matcher.
/// This function validates the values and throws an error if not met requirements.
pub fn create_config(matches: &ArgMatches) -> Result<Config, &'static str>  {
    let width = matches.value_of("width").unwrap_or(DEFAULT_WIDTH);
    let height = matches.value_of("height").unwrap_or(DEFAULT_HEIGHT);
    let sleep = matches.value_of("sleep").unwrap_or(DEFAULT_SLEEP);
    let ratio = matches.value_of("ratio").unwrap_or(DEFAULT_RATIO);

    validate_config(width, height, sleep, ratio)
}

fn validate_config(width: &str, height: &str, sleep: &str, ratio: &str) -> Result<Config, &'static str> {
    let width = width.parse::<usize>().expect("Not negative number expected as width!");
    // TODO check for minimum size.

    let height = height.parse::<usize>().expect("Not negative number expected as height!");
    // TODO check for minimum size.

    let sleep = sleep.parse::<u64>().expect("Not negative number expected as sleep!");
    // TODO Check for not less than 1

    let ratio = ratio.parse::<u32>().expect("Not negative number expected as sleep!");
    // TODO Check for not less than 1

    Ok(Config::new(width, height, sleep, ratio))
}

/// This struct holds the configuration for the game.
#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    width: usize,
    height: usize,
    sleep: u64,
    ratio: u32,
}

impl Config {
    fn new(width: usize, height: usize, sleep: u64, ratio: u32) -> Config {
        Config { width, height, sleep, ratio }
    }

    /// Get the width of the space the population have.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Get the height of the space the population have.
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Seconds to sleep between the population's iterations.
    pub fn get_sleep(&self) -> Duration {
        time::Duration::from_secs(self.sleep)
    }

    /// The probability ration for generating initial population.
    pub fn get_ratio(&self) -> u32 {
        self.ratio
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Width:     {:5}, Height: {:5}, Sleep: {:5}, Ration: {:5}", self.width, self.height, self.sleep, self.ratio)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn fmt() {
        let sut = Config::new(42, 23, 5, 3);

        assert_that!(
            format!("{}", sut),
            is(equal_to(String::from("Width:        42, Height:    23, Sleep:     5, Ration:     3"))));
    }
}