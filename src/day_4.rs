use crate::{
    Result,
    grid::{ADJACENT, Grid, Point},
    print_results, read_input,
};

pub fn run() -> Result<()> {
    let input = read_input("day4")?;

    let first = part_1(&input);
    let second = part_2(&input);

    print_results(first, second);

    Ok(())
}

fn part_1(input: &str) -> usize {
    let mut grid = Grid::new(input);

    removable_rolls(&mut grid).len()
}

fn part_2(input: &str) -> usize {
    let mut grid = Grid::new(input);
    let mut count = 0;

    loop {
        let to_remove = removable_rolls(&mut grid);

        if to_remove.is_empty() {
            break;
        }

        count += to_remove.len();
        for point in to_remove {
            grid[point] = b'.';
        }
    }

    count
}

fn removable_rolls(grid: &mut Grid) -> Vec<Point> {
    let mut to_remove = Vec::<Point>::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x as i32, y as i32);

            if grid.get(point) == Some(b'@') {
                let count = ADJACENT
                    .iter()
                    .map(|a| *a + point)
                    .filter(|&p| grid.get(p).is_some_and(|v| v == b'@'))
                    .count();

                if count < 4 {
                    to_remove.push(point);
                }
            }
        }
    }

    to_remove
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
        assert_eq!(part_1(EXAMPLE), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part_2(EXAMPLE), 43);
    }
}
