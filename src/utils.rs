use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt::Display;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(dead_code)]
pub fn print_vec<T>(vec: &Vec<T>) 
where T: Display
{
    for item in vec {
        print!("{} ", item);
    }
    println!();
}
