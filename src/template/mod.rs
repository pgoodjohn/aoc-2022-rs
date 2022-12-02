use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(arg_required_else_help(true))]
pub struct TemplateCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<TemplateCommands>,
}

#[derive(Subcommand)]
pub enum TemplateCommands {
    One {},
    Two {},
}

pub fn command(command: &TemplateCommand) -> Result<String, Box<dyn std::error::Error>> {
    match command.command {
        Some(TemplateCommands::One {}) => part_one(),
        Some(TemplateCommands::Two {}) => part_two(),
        None => {
            panic!("Handled by clap");
        }
    }
}

fn part_one() -> Result<String, Box<dyn std::error::Error>> {
    todo!("Implement part one");
}

fn part_two() -> Result<String, Box<dyn std::error::Error>> {
    todo!("Implement part two");
}
