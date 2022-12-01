use log;
use std::io::prelude::*;

pub fn run_part_one() -> Result<String, Box<dyn std::error::Error>> {

    log::info!("Day One Part One");

    let mut all_totals = parse_input()?;

    all_totals.sort();
    let highest_total = all_totals.clone().pop();

    match highest_total {
        Some(r) => {
            log::info!("The Elf carrying the most calories is carrying {}", r);
            Ok(r.to_string())
        },
        None => {
            log::error!("Could not get a value from the totals vectors.");
            Err(String::from("Could not get a value from the totals vector").into())
        }
    }
}

pub fn run_part_two() -> Result<String, Box<dyn std::error::Error>> {

    log::info!("Day One Part Two");

    let mut all_totals = parse_input()?;

    all_totals.sort();

    let mut top_three_totals: i32 = 0;

    for _ in 0..3 {
        // Don't want to be bothered with matching here :D 
        let current_max = all_totals.pop().unwrap();
        top_three_totals = top_three_totals + current_max;
    }

    log::info!("The top three elves are carrying {}", top_three_totals);

    Ok(top_three_totals.to_string())
}

fn parse_input() -> Result<Vec<i32>, Box<dyn::std::error::Error>> {

    let input_file = std::fs::File::open("./one/input.txt")?;
    let reader = std::io::BufReader::new(input_file);

    let mut current_total: i32 = 0;

    let mut all_totals: Vec<i32> = Vec::new();;

    for line in reader.lines() {
        log::debug!("{:?}", line);
        match line? {
            s if s != String::from("") => {
                log::info!("Found {}", s);
                current_total = current_total + s.parse::<i32>()?;
            }
            _ => {
                log::info!("Found empty line, adding current total {} to totals vector", current_total);
                all_totals.push(current_total);
                current_total = 0;
            }
        }
    }

    log::debug!("{:?}", all_totals);

    Ok(all_totals)
}
