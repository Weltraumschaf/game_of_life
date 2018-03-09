extern crate game_of_life;
extern crate clap;

use std::thread;
use clap::{Arg, App};
use game_of_life::config::create_config;
use game_of_life::create_initial_population;
use game_of_life::screen::{clear, print_header};
use game_of_life::status::Status;

/// The main entry point of the binary.
///
/// This create follows the paradigm of an executable library: This means that the whole application
/// is in the library part of the crate (and so can be used as a dependency). The main module only
/// provides the main function with simple code which delegates to the library part.
fn main() {
    let matches = App::new("Game of Life")
        .version("1.0.0")
        .author("Sven Strittmatter <ich@weltraumschaf.de>")
        .about("This is a Game of Life implementation.")
        .arg(Arg::with_name("width")
            .long("width")
            .value_name("WIDTH")
            .help("Sets width of the population space. Default is 40.")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .long("height")
            .value_name("HEIGHT")
            .help("Sets height of the population space. Default is 20.")
            .takes_value(true))
        .arg(Arg::with_name("sleep")
            .long("sleep")
            .value_name("SLEEP")
            .help("Sets sleep time in seconds between the population iterations. Default is 1.")
            .takes_value(true))
        .arg(Arg::with_name("ratio")
            .long("ratio")
            .value_name("RATIO")
            .help("A probability ratio used for the initial cell generation. Default is 4.")
            .takes_value(true))
        .get_matches();
    let config = create_config(&matches);

    let mut population = create_initial_population(&config);
    let mut previous_status = population.get_status();

    loop {
        clear();
        print_header();
        println!("{}", &config);
        println!("{}", population.get_status());
        println!();
        print!("{}", population);
        population = population.next_generation();

        if previous_status.is_population_unchanged(population.get_status()) {
            println!("Population is stuck! No more evolution...");
            break;
        }

        previous_status = population.get_status();
        thread::sleep(config.get_sleep());
    }
}

