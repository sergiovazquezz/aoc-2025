use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut dial = Dial::new();
    let moves_file = File::open("./inputs/day1.txt")?;
    let reader = BufReader::new(moves_file);

    for line in reader.lines().map_while(|l| l.ok()) {
        let trimmed_line = line.trim();

        if trimmed_line.len() >= 2 {
            dial.handle_rotation(trimmed_line);
        }
    }

    println!("Part 1: {}", dial.final_code);

    Ok(())
}

enum Rotation {
    Left(i32),
    Right(i32),
}

struct Dial {
    current_pos: i32,
    final_code: u16,
}

impl Dial {
    const MIN_POS: i32 = 0;
    const MAX_POS: i32 = 99;
    const RANGE: i32 = Self::MAX_POS - Self::MIN_POS + 1;
    const INITIAL_POS: i32 = 50;

    fn new() -> Dial {
        Dial {
            current_pos: Self::INITIAL_POS,
            final_code: 0,
        }
    }

    fn handle_rotation(&mut self, rotation_str: &str) {
        let rotation = Dial::parse_rotation(rotation_str);

        if let Some(rotation) = rotation {
            self.rotate(rotation);

            if self.is_start() {
                self.final_code += 1;
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
        let pos = self.current_pos;
        let range = Self::RANGE;

        self.current_pos = match rotation {
            Rotation::Left(steps) => (pos - steps).rem_euclid(range),
            Rotation::Right(steps) => (pos + steps).rem_euclid(range),
        }
    }

    fn is_start(&self) -> bool {
        if self.current_pos == Self::MIN_POS {
            return true;
        }

        false
    }
}
