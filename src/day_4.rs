use crate::{
    Result,
    grid::{ADJACENT, Grid, Point},
    print_results, read_input,
};

pub fn run() -> Result<()> {
    let input = read_input("day4")?;

    let first = part_1(&input)?;

    print_results(first, "TODO");

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
