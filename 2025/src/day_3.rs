use crate::read_input;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("day3")?;
    let mut results = (0u32, 0u32);

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let bank: Vec<u8> = line
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|b| b - b'0')
            .collect();

        results.0 += run_part_1(&bank)?;
    }

    println!("Part 1: {}", results.0);

    Ok(())
}

fn run_part_1(bank: &[u8]) -> Result<u32, Box<dyn std::error::Error>> {
    let (mut first, mut second) = ((0usize, 0u8), (0usize, 0u8));
    let mut new_first: bool = false;

    for (i, &v) in bank.iter().enumerate() {
        if v > first.1 && i < bank.len() - 1 {
            first = (i, v);
            new_first = true;
        } else if new_first || (v > second.1 && i > first.0) {
            second = (i, v);
            new_first = false;
        }
    }

    let bank_joltage = (first.1 * 10 + second.1) as u32;

    Ok(bank_joltage)
}
