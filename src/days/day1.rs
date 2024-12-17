use std::iter::zip;
use crate::utils::read_lines;

pub fn solve() {
    let mut col1: Vec<u32> = Vec::new();
    let mut col2: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("./input_files/p1_input") {
        for line in lines.flatten() {
            let words = line.split("   ");
            for (i, word) in words.into_iter().enumerate() {
                if i == 0 {
                    col1.push(word.parse::<u32>().unwrap());
                } else if i == 1 {
                    col2.push(word.parse::<u32>().unwrap());
                } else {
                    continue;
                }
            }
        }
    } else {
        println!("Cant read file");
    }

    col1.sort();
    col2.sort();

    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;

    for (x, y) in zip(col1.iter(), col2.iter()) {
        if x > y {
            sum1 += x - y;
        }
        else {
            sum1 += y - x;
        }
    }
    println!("Part 1: {sum1}");

    for x in col1 {
        let count = col2.iter().filter(|&&val| val == x).count() as u32;
        sum2 += x * count;
    }

    println!("Part 2: {sum2}");

}
