use clap::{Parser, Subcommand};
use log;
use plogger;

#[derive(Parser)]
#[clap(about, arg_required_else_help(true))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(short, long)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    One,
}

mod one;

fn main() {
    let cli = Cli::parse();

    plogger::init(cli.debug);

    log::info!("Welcome to the Advent of Code 2022 Solutions Program");

    match cli.command {
        Some(Commands::One) => {
            match one::run_part_one() {
                Ok(r) => {
                    log::info!("Day One Part One successful! Result is: {}", r);
                }
                Err(e) => {
                    log::error!("Something went wrong: {:?}", e);
                }
            }

            match one::run_part_two() {
                Ok(r) => {
                    log::info!("Day One Part Two successful! Result is: {}", r);
                }
                Err(e) => {
                    log::error!("Something went wrong: {:?}", e);
                }
            }
        },
        None => {} // Handled by Clap
    }
}
