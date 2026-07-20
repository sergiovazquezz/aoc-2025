use crate::{
    Result,
    grid::{ADJACENT, Grid, Point},
    read_input,
};

pub fn run() -> Result<()> {
    let input = read_input("day4")?;
    let mut results = (0usize, 0usize);

    results.0 = part_1(&input)?;

    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);

    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    let grid = Grid::new(input);
    let mut result = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x as i32, y as i32);

            if grid[point] == b'@' {
                let count = ADJACENT
                    .iter()
                    .map(|a| *a + point)
                    .filter(|&p| grid.contains(p) && grid[p] == b'@')
                    .count();

                if count < 4 {
                    result += 1;
                }
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = concat!(
        "..@@.@@@@.\n",
        "@@@.@.@.@@\n",
        "@@@@@.@.@@\n",
        "@.@@@@..@.\n",
        "@@.@@@@.@@\n",
        ".@@@@@@@.@\n",
        ".@.@.@.@@@\n",
        "@.@@@.@@@@\n",
        ".@@@@@@@@.\n",
        "@.@.@@@.@.",
    );

    #[test]
    fn part1_example() {
        assert_eq!(part_1(EXAMPLE).unwrap(), 13);
    }
}
