use std::env;

mod day_1;

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
        2..NUM_DAYS => todo!("I have not yet completed these days"),
        _ => unreachable!(),
    }

    Ok(())
}
