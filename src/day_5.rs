use std::collections::HashSet;

use crate::read_input;

type Range = (usize, usize);

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day5")?;
    let mut results = (0usize, 0u32);

    let (fresh_ids, available_ids) = get_ids(&input);

    results.0 = run_part_1(fresh_ids, available_ids);

    println!("Part 1: {}", results.0);

    Ok(())
}

fn run_part_1(fresh_ids: Vec<Range>, available_ids: HashSet<usize>) -> usize {
    available_ids
        .iter()
        .filter(|&&id| fresh_ids.iter().any(|f| f.0 <= id && f.1 >= id))
        .count()
}

fn run_part_2(fresh_ids: Vec<Range>) -> usize {
    todo!()
}

fn get_ids(input: &str) -> (Vec<Range>, HashSet<usize>) {
    let (ranges_part, ids_part) = input
        .split_once("\n\n")
        .expect("expected blank line separating sections");

    let fresh_ids: Vec<Range> = ranges_part
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .collect();

    let available_ids: HashSet<usize> = ids_part
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    (fresh_ids, available_ids)
}
