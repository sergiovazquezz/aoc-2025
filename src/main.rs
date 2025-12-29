use std::{env, fs::File, io::Read, path::PathBuf};

mod day_1;
mod day_2;
mod day_3;

const INPUT_PATH: &str = "./inputs";

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        2 => day_2::run()?,
        3 => day_3::run()?,
        4..NUM_DAYS => todo!("I have not yet completed these days"),
        _ => unreachable!(),
    }

    Ok(())
}

pub fn read_input(day: &str) -> Result<String, std::io::Error> {
    let file_path = PathBuf::from(INPUT_PATH).join(day).with_extension("txt");
    let mut file = File::open(file_path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)
}
