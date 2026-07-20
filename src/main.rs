use std::{env, fs, path::PathBuf};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

mod grid;

const INPUT_PATH: &str = "./inputs";

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    const NUM_DAYS: u8 = 12;

    if args.len() != 2 {
        return Err("Invalid number of arguments, only 1 should be supplied".into());
    }

    let day = args[1].parse::<u8>()?;

    if !(1..=NUM_DAYS).contains(&day) {
        return Err("Not a valid day, there are 12 days".into());
    }

    match day {
        1 => day_1::run()?,
        3 => day_3::run()?,
        4 => day_4::run()?,
        5 => day_5::run()?,
        6..NUM_DAYS | 2 => todo!("This day does not have a solution yet!"),
        _ => unreachable!(),
    }

    Ok(())
}

#[inline]
pub fn read_input(day: &str) -> Result<String> {
    let file_path = PathBuf::from(INPUT_PATH).join(day).with_extension("txt");

    let content = fs::read_to_string(file_path)?;

    Ok(content)
}
