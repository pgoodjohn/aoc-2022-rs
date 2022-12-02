use lazy_static::lazy_static;
use log;
use regex::Regex;
use std::io::prelude::*;
use std::str::FromStr;
use std::string::ToString;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(arg_required_else_help(true))]
pub struct DayTwoCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<DayTwoCommands>,
}

#[derive(Subcommand)]
pub enum DayTwoCommands {
    One {},
    Two {},
}

pub fn command(command: &DayTwoCommand) -> Result<String, Box<dyn std::error::Error>> {
    match command.command {
        Some(DayTwoCommands::One {}) => part_one(),
        Some(DayTwoCommands::Two {}) => part_two(),
        None => {
            panic!("Handled by clap");
        }
    }
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        match self {
            Shape::Rock => "X".to_string(),
            Shape::Paper => "Y".to_string(),
            Shape::Scissors => "Z".to_string(),
        }
    }
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err("Could not understand shape input".into()),
        }
    }
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn fight(player: Shape, npc: Shape) -> Result<CombatResult, String> {
        match player {
            Shape::Rock => match npc {
                Shape::Rock => Ok(CombatResult::Draw),
                Shape::Paper => Ok(CombatResult::NPCWins),
                Shape::Scissors => Ok(CombatResult::PlayerWins),
            },
            Shape::Paper => match npc {
                Shape::Rock => Ok(CombatResult::PlayerWins),
                Shape::Paper => Ok(CombatResult::Draw),
                Shape::Scissors => Ok(CombatResult::NPCWins),
            },
            Shape::Scissors => match npc {
                Shape::Rock => Ok(CombatResult::NPCWins),
                Shape::Paper => Ok(CombatResult::PlayerWins),
                Shape::Scissors => Ok(CombatResult::Draw),
            },
        }
    }

    fn from_enemy_and_expected_result(
        enemy: &Shape,
        expected_result: ExpectedResult,
    ) -> Result<Self, String> {
        match enemy {
            Shape::Paper => match expected_result {
                ExpectedResult::Draw => Ok(Shape::Paper),
                ExpectedResult::NPCWins => Ok(Shape::Rock),
                ExpectedResult::PlayerWins => Ok(Shape::Scissors),
            },
            Shape::Rock => match expected_result {
                ExpectedResult::Draw => Ok(Shape::Rock),
                ExpectedResult::NPCWins => Ok(Shape::Scissors),
                ExpectedResult::PlayerWins => Ok(Shape::Paper),
            },
            Shape::Scissors => match expected_result {
                ExpectedResult::Draw => Ok(Shape::Scissors),
                ExpectedResult::NPCWins => Ok(Shape::Paper),
                ExpectedResult::PlayerWins => Ok(Shape::Rock),
            },
        }
    }
}

#[derive(Debug)]
enum CombatResult {
    PlayerWins,
    NPCWins,
    Draw,
}

impl CombatResult {
    fn score(&self) -> i32 {
        match self {
            CombatResult::PlayerWins => 6,
            CombatResult::Draw => 3,
            CombatResult::NPCWins => 0,
        }
    }
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"(?P<enemy>[ABC])\s(?P<player>[XYZ])").unwrap();
}

pub fn part_one() -> Result<String, Box<dyn std::error::Error>> {
    log::info!("Day Two Part One");

    let mut player_total_score: i32 = 0;
    let mut fights: i32 = 0;

    let input_file = std::fs::File::open("./src/two/input.txt")?;
    let reader = std::io::BufReader::new(input_file);

    for line in reader.lines() {
        match line.unwrap() {
            line => {
                log::debug!("{:?}", &line);
                let captures = LINE_REGEX.captures(&line.as_str());
                log::debug!("{:?}", &captures);

                if let Some(captures) = captures {
                    let enemy = Shape::from_str(captures.name("enemy").unwrap().as_str())?;
                    let player = Shape::from_str(captures.name("player").unwrap().as_str())?;

                    log::debug!("Enemy: {:?} - Player: {:?}", enemy, player);

                    player_total_score = player_total_score + player.score();

                    let combat_result = Shape::fight(player, enemy)?;

                    log::debug!("Fight result was {:?}", combat_result);

                    player_total_score = player_total_score + combat_result.score();

                    fights = fights + 1;
                }
            }
        }
    }

    log::info!(
        "Accumulated score of {} across {} fights",
        &player_total_score,
        &fights
    );

    Ok(player_total_score.to_string())
}

#[derive(Debug)]
enum ExpectedResult {
    PlayerWins,
    NPCWins,
    Draw,
}

impl FromStr for ExpectedResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(ExpectedResult::NPCWins),
            "Y" => Ok(ExpectedResult::Draw),
            "Z" => Ok(ExpectedResult::PlayerWins),
            _ => Err("Could not understand shape input".into()),
        }
    }
}

pub fn part_two() -> Result<String, Box<dyn std::error::Error>> {
    log::info!("Day Two Part Two");

    let mut player_total_score: i32 = 0;
    let mut fights: i32 = 0;

    let input_file = std::fs::File::open("./src/two/input.txt")?;
    let reader = std::io::BufReader::new(input_file);

    for line in reader.lines() {
        match line.unwrap() {
            line => {
                log::debug!("{:?}", &line);
                let captures = LINE_REGEX.captures(&line.as_str());
                log::debug!("{:?}", &captures);

                if let Some(captures) = captures {
                    let enemy = Shape::from_str(captures.name("enemy").unwrap().as_str())?;
                    let expected_result =
                        ExpectedResult::from_str(captures.name("player").unwrap().as_str())?;

                    let player = Shape::from_enemy_and_expected_result(&enemy, expected_result)?;

                    log::debug!("Enemy: {:?} - Player: {:?}", enemy, player);

                    player_total_score = player_total_score + player.score();

                    let combat_result = Shape::fight(player, enemy)?;

                    log::debug!("Fight result was {:?}", combat_result);

                    player_total_score = player_total_score + combat_result.score();

                    fights = fights + 1;
                }
            }
        }
    }

    log::info!(
        "Accumulated score of {} across {} fights",
        &player_total_score,
        &fights
    );

    Ok(player_total_score.to_string())
}
