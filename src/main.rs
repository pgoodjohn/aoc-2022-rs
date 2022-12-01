use plogger;
use log;

mod one;

fn main() {
    plogger::init(true);

    log::info!("Welcome to the Advent of Code 2022 Solutions Program");

    match one::run_part_one() {
        Ok(r) => {
            log::info!("Day One Part One successful! Result is: {}", r);
        },
        Err(e) => {
            log::error!("Something went wrong: {:?}", e);
        }
    }

    match one::run_part_two() {
        Ok(r) => {
            log::info!("Day One Part Two successful! Result is: {}", r);
        },
        Err(e) => {
            log::error!("Something went wrong: {:?}", e);
        }
    }
}
