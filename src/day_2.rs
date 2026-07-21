use crate::{Result, print_results, read_input};

type Range = [u64; 2];

pub fn run() -> Result<()> {
    let input = read_input("day2")?;
    let ranges = parse_ranges(&input);

    let first = part_1(&ranges);
    let second = part_2(&ranges);

    print_results(first, second);

    Ok(())
}

fn part_1(ranges: &[Range]) -> u64 {
    let mut sum = 0;

    for range in ranges {
        let start = range[0];
        let end = range[1];

        let num_digits_start = start.ilog10() + 1;
        let num_digits_end = end.ilog10() + 1;

        if num_digits_start == num_digits_end && num_digits_start % 2 != 0 {
            continue;
        }

        for num in start..=end {
            let num_digits = num.ilog10() + 1;

            if num_digits % 2 != 0 {
                continue;
            }

            let divisor = 10u64.pow(num_digits / 2);

            let left = num / divisor;
            let right = num % divisor;

            if left == right {
                sum += num;
            }
        }
    }

    sum
}

fn part_2(ranges: &[Range]) -> u64 {
    let mut sum = 0;

    for range in ranges {
        let start = range[0];
        let end = range[1];

        for num in start..=end {
            let num_digits = num.ilog10() + 1;

            for chunk_len in 1..=num_digits / 2 {
                if num_digits % chunk_len != 0 {
                    continue;
                }

                let divisor = 10u64.pow(chunk_len);

                let expected_chunk = num % divisor;
                let mut remaining = num / divisor;

                let mut matches = true;

                while remaining != 0 {
                    if remaining % divisor != expected_chunk {
                        matches = false;
                        break;
                    }

                    remaining /= divisor;
                }

                if matches {
                    sum += num;
                    break;
                }
            }
        }
    }

    sum
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input
        .trim()
        .split(',')
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .map(|r| {
            let (start, end) = r.split_once('-').unwrap();
            [start.parse().unwrap(), end.parse().unwrap()]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn parse_example() {
        assert_eq!(parse_ranges(EXAMPLE).len(), 11)
    }

    #[test]
    fn part1_example() {
        let ranges = parse_ranges(EXAMPLE);
        assert_eq!(part_1(&ranges), 1227775554);
    }

    #[test]
    fn part2_example() {
        let ranges = parse_ranges(EXAMPLE);
        assert_eq!(part_2(&ranges), 4174379265);
    }
}
