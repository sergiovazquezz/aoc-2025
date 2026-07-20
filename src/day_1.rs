use crate::{Result, print_results, read_input};

pub fn run() -> Result<()> {
    let input = read_input("day1")?;

    let first = run_part_1(&input)?;
    let second = run_part_2(&input)?;

    print_results(first, second);

    Ok(())
}

fn run_part_1(input: &str) -> Result<u16> {
    let mut dial = Dial::new();

    for line in input.lines() {
        let line = line.trim();

        if line.len() < 2 {
            continue;
        }

        dial.handle_rotation(line);
    }

    Ok(dial.code)
}

fn run_part_2(input: &str) -> Result<u16> {
    let mut dial = Dial::new();

    for line in input.lines() {
        let line = line.trim();

        if line.len() < 2 {
            continue;
        }

        let rotation = Dial::parse_rotation(line);

        match rotation {
            Some(Rotation::Left(steps)) => {
                for _ in 0..steps {
                    dial.position += 1;

                    if dial.position == Dial::MAX_POS + 1 {
                        dial.position = Dial::MIN_POS;
                    }

                    if dial.is_start() {
                        dial.code += 1;
                    }
                }
            }
            Some(Rotation::Right(steps)) => {
                for _ in 0..steps {
                    dial.position -= 1;

                    if dial.position == Dial::MIN_POS - 1 {
                        dial.position = Dial::MAX_POS;
                    }

                    if dial.is_start() {
                        dial.code += 1;
                    }
                }
            }
            _ => (),
        }
    }

    Ok(dial.code)
}

enum Rotation {
    Left(i32),
    Right(i32),
}

struct Dial {
    position: i32,
    code: u16,
}

impl Dial {
    const MIN_POS: i32 = 0;
    const MAX_POS: i32 = 99;
    const RANGE: i32 = Self::MAX_POS - Self::MIN_POS + 1;
    const INITIAL_POS: i32 = 50;

    fn new() -> Dial {
        Dial {
            position: Self::INITIAL_POS,
            code: 0,
        }
    }

    fn handle_rotation(&mut self, rotation_str: &str) {
        let rotation = Dial::parse_rotation(rotation_str);

        if let Some(rotation) = rotation {
            self.rotate(rotation);

            if self.is_start() {
                self.code += 1;
            }
        }
    }

    fn parse_rotation(rotation_str: &str) -> Option<Rotation> {
        let mut rotation_chars = rotation_str.chars();

        let direction = rotation_chars.next()?;
        let steps = rotation_chars.as_str().parse::<i32>().ok()?;

        let rotation = match direction {
            'L' => Rotation::Left(steps),
            'R' => Rotation::Right(steps),
            _ => return None,
        };

        Some(rotation)
    }

    fn rotate(&mut self, rotation: Rotation) {
        let pos = self.position;
        let range = Self::RANGE;

        let unnormalized_position: i32 = match rotation {
            Rotation::Left(steps) => pos - steps,
            Rotation::Right(steps) => pos + steps,
        };

        self.position = unnormalized_position.rem_euclid(range);
    }

    fn is_start(&self) -> bool {
        if self.position == Self::MIN_POS {
            return true;
        }

        false
    }
}
