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
pub fn create_config(matches: &ArgMatches) -> Result<Config, String> {
    let width = matches.value_of("width").unwrap_or(DEFAULT_WIDTH);
    let height = matches.value_of("height").unwrap_or(DEFAULT_HEIGHT);
    let sleep = matches.value_of("sleep").unwrap_or(DEFAULT_SLEEP);
    let ratio = matches.value_of("ratio").unwrap_or(DEFAULT_RATIO);

    validate_config(width, height, sleep, ratio)
}

fn validate_config(width: &str, height: &str, sleep: &str, ratio: &str) -> Result<Config, String> {
    let width = match width.parse::<usize>() {
        Ok(w) => {
            if w < 1 {
                return Err(error_bad_option_to_small("width", 1))
            }

            w
        },
        Err(_) => return Err(error_bad_option_not_number("width")),
    };

    let height = match height.parse::<usize>() {
        Ok(h) => {
            if h < 1 {
                return Err(error_bad_option_to_small("height", 1))
            }

            h
        },
        Err(_) => return Err(error_bad_option_not_number("height")),
    };

    let sleep = match sleep.parse::<u64>() {
        Ok(s) => {
            if s < 1 {
                return Err(error_bad_option_to_small("sleep", 1))
            }

            s
        },
        Err(_) => return Err(error_bad_option_not_number("sleep")),
    };

    let ratio = match ratio.parse::<u32>() {
        Ok(r) => {
            if r < 1 {
                return Err(error_bad_option_to_small("ratio", 1))
            }

            r
        },
        Err(_) => return Err(error_bad_option_not_number("ratio")),
    };

    Ok(Config::new(width, height, sleep, ratio))
}

static OPTION_ERROR_PREFIX: &'static str = "Bad option:";

fn error_bad_option_not_number(name: &str) -> String {
    format!("{} Not negative number expected as option '--{}'!", OPTION_ERROR_PREFIX, name)
}

fn error_bad_option_to_small(name: &str, min: usize) -> String {
    format!("{} Too small value for option '--{}' given! Minimum is {}.", OPTION_ERROR_PREFIX, name, min)
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
        write!(f, "Width:     {:5}, Height: {:5}, Sleep: {:5}, Ratio: {:5}", self.width, self.height, self.sleep, self.ratio)
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
            is(equal_to(String::from("Width:        42, Height:    23, Sleep:     5, Ratio:     3"))));
    }

    #[test]
    fn validate_config_width_is_not_usize() {
        let result = validate_config("foo", "1", "1", "1");

        assert_that!(
            result,
            is(equal_to(Err(
                String::from("Bad option: Not negative number expected as option '--width'!")))));
    }

    #[test]
    fn validate_config_width_is_too_small() {
        let result = validate_config("0", "1", "1", "1");

        assert_that!(
            result,
            is(equal_to(Err(
                String::from("Bad option: Too small value for option '--width' given! Minimum is 1.")))));
    }

    #[test]
    fn validate_config_height_is_not_usize() {
        let result = validate_config("1", "foo", "1", "1");

        assert_that!(
            result,
            is(equal_to(Err(
                String::from("Bad option: Not negative number expected as option '--height'!")))));
    }

    #[test]
    fn validate_config_height_is_to_small() {
        let result = validate_config("1", "0", "1", "1");

        assert_that!(
            result,
            is(equal_to(Err(
                String::from("Bad option: Too small value for option '--height' given! Minimum is 1.")))));
    }

    #[test]
    fn validate_config_sleep_is_not_usize() {
        let result = validate_config("1", "1", "foo", "1");

        assert_that!(
            result,
            is(equal_to(Err(
                String::from("Bad option: Not negative number expected as option '--sleep'!")))));
    }

    #[test]
    fn validate_config_sleep_is_to_small() {
        let result = validate_config("1", "1", "0", "1");

        assert_that!(
            result,
            is(equal_to(Err(
                String::from("Bad option: Too small value for option '--sleep' given! Minimum is 1.")))));
    }

    #[test]
    fn validate_config_ratio_is_not_usize() {
        let result = validate_config("1", "1", "1", "foo");

        assert_that!(
            result,
            is(equal_to(Err(
                String::from("Bad option: Not negative number expected as option '--ratio'!")))));
    }

    #[test]
    fn validate_config_ratio_is_to_small() {
        let result = validate_config("1", "1", "1", "0");

        assert_that!(
            result,
            is(equal_to(Err(
                String::from("Bad option: Too small value for option '--ratio' given! Minimum is 1.")))));
    }

    #[test]
    fn validate_config_with_sane_values() {
        let result = match validate_config("1", "2", "3", "4") {
            Ok(r) => r,
            Err(_) => panic!("Expected Ok(Config) as result!"),
        };

        assert_that!(result.get_width(), is(equal_to(1)));
        assert_that!(result.get_height(), is(equal_to(2)));
        assert_that!(result.get_sleep(), is(equal_to(time::Duration::from_secs(3))));
        assert_that!(result.get_ratio(), is(equal_to(4)));
    }
}
