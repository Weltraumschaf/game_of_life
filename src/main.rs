extern crate game_of_life;
extern crate clap;

use std::process;
use clap::{Arg, App};
use game_of_life::config::*;
use game_of_life::*;

/// The main entry point of the binary.
///
/// This crate follows the paradigm of an executable library: This means that the whole application
/// is in the library part of the crate (and so can be used as a dependency). The main module only
/// provides the main function with simple code which delegates to the library part.
fn main() {
    let matches = App::new(game_of_life::APPLICATION_NAME)
        .version(game_of_life::APPLICATION_VERSION)
        .author(game_of_life::APPLICATION_AUTHOR)
        .about(game_of_life::APPLICATION_DESCRIPTION)
        .arg(Arg::with_name("width")
            .long("width")
            .value_name("WIDTH")
            .help(
                &format!(
                    "Sets width of the population space. Default is {}.",
                    config::DEFAULT_WIDTH))
            .takes_value(true))
        .arg(Arg::with_name("height")
            .long("height")
            .value_name("HEIGHT")
            .help(
                &format!(
                    "Sets height of the population space. Default is {}.",
                    config::DEFAULT_HEIGHT))
            .takes_value(true))
        .arg(Arg::with_name("sleep")
            .long("sleep")
            .value_name("SLEEP")
            .help(
                &format!(
                    "Sets sleep time in seconds between the population iterations. Default is {}.",
                    config::DEFAULT_SLEEP))
            .takes_value(true))
        .arg(Arg::with_name("ratio")
            .long("ratio")
            .value_name("RATIO")
            .help(
                &format!(
                    "A probability ratio used for the initial cell generation. Default is {}.",
                    config::DEFAULT_RATIO))
            .takes_value(true))
        .get_matches();

    let config = create_config(&matches).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    run_game(config);
}

