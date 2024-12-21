mod days;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Please provide a day number");
        return;
    }

    let day = args[1].parse::<u32>().unwrap_or(0);
    
    match day {
        1 => days::day1::solve(),
        2 => days::day2::solve(),
        3 => days::day3::solve(),
        4 => days::day4::solve(),
        _ => println!("Day {} not implemented yet", day),
    }
}
