use std::collections::HashSet;

use crate::{Result, print_results, read_input};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: usize,
    end: usize,
}

pub fn run() -> Result<()> {
    let input = read_input("day5")?;

    let (fresh_ids, available_ids) = get_ids(&input);

    let first = run_part_1(&fresh_ids, available_ids);
    let second = run_part_2(fresh_ids);

    print_results(first, second);

    Ok(())
}

fn run_part_1(fresh_ids: &[Range], available_ids: HashSet<usize>) -> usize {
    available_ids
        .iter()
        .filter(|&&id| fresh_ids.iter().any(|f| f.start <= id && f.end >= id))
        .count()
}

fn run_part_2(mut fresh_ids: Vec<Range>) -> usize {
    fresh_ids.sort_by_key(|k| k.start);

    // for range in sorted_ids {}

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
