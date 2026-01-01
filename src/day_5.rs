use std::collections::HashSet;

use crate::read_input;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day5")?;
    let mut results = (0u32, 0u32);

    let fresh_ids = get_fresh_ids(&input);
    let current_ids 

    results.0 = run_part_1()?;

    Ok(())
}

fn run_part_1() -> Result<u32, Box<dyn std::error::Error>> {
    
}

fn split_input(input: &str) -> (String, String) {
    let mut found_empty_line: bool = false;
    let mut fresh_ids_string = String::new();
    let mut all_ids_string = String::new();

    for line in input.lines() {
        let line = line.trim();

        if !found_empty_line && line.is_empty() {
            found_empty_line = true;
        }

        if !found_empty_line {
           fresh_ids_string.push_str(line);
        } else {
            all_ids_string.push_str(line);
        }
    }


    (fresh_ids_string, all_ids_string)
}

fn get_fresh_ids(input: &str) -> HashSet<u32> {
    let mut fresh_ids: HashSet<u32> = HashSet::new();

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once("-").unwrap();

        let start = start.parse::<u32>().unwrap();
        let end = end.parse::<u32>().unwrap();

        fresh_ids.extend(start..=end);
    }

    fresh_ids
}
