use std::collections::HashSet;

use crate::read_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: usize,
    end: usize,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day5")?;
    let mut results = (0usize, 0usize);

    let (fresh_ids, available_ids) = get_ids(&input);

    results.0 = run_part_1(&fresh_ids, available_ids);
    results.1 = run_part_2(&fresh_ids);

    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);

    Ok(())
}

fn run_part_1(fresh_ids: &[Range], available_ids: HashSet<usize>) -> usize {
    available_ids
        .iter()
        .filter(|&&id| fresh_ids.iter().any(|f| f.start <= id && f.end >= id))
        .count()
}

fn run_part_2(fresh_ids: &[Range]) -> usize {
    let mut sorted_ids: Vec<Range> = fresh_ids.to_vec();
    sorted_ids.sort_by_key(|k| k.start);

    let mut final_ids: Vec<Range> = Vec::new().append(sorted_ids.first().unwrap());

    for range in sorted_ids {}

    0usize
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
            Range {
                start: start.parse::<usize>().unwrap(),
                end: end.parse::<usize>().unwrap(),
            }
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
