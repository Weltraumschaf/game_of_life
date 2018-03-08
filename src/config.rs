use clap::ArgMatches;

/// Creates a new config from the arguments matcher.
/// This function validates the values and throws an error if not met requirements.
pub fn create_config(matches: &ArgMatches) -> Config {
    let width = matches.value_of("width").unwrap_or("20");
    let width = width.parse::<usize>().expect("Not negative number expected as width!");
    // TODO check for minimum size.

    let height = matches.value_of("height").unwrap_or("20");
    let height = height.parse::<usize>().expect("Not negative number expected as height!");
    // TODO check for minimum size.

    let sleep = matches.value_of("sleep").unwrap_or("1");
    let sleep = sleep.parse::<u64>().expect("Not negative number expected as sleep!");
    // TODO Check for not less than 1

    Config { width, height, sleep }
}

/// This struct holds the configuration for the game.
#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    width: usize,
    height: usize,
    sleep: u64,
}

impl Config {
    /// Get the width of the space the population have.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Get the height of the space the population have.
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Seconds to sleep between the population's iterations.
    pub fn get_sleep(&self) -> u64 {
        self.sleep
    }
}
