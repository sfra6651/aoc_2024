

use crate::utils::read_lines;

pub fn solve() {
    let lines = read_lines("./input_files/p2_input").unwrap();

    let mut safe_count = 0;

    for line in lines.flatten() {
        let words:Vec<&str> = line.split(" ").collect();
        if is_safe(&words) {
            safe_count += 1;
        } else {
            //part 2 condition
            for (idx, word) in words.iter().enumerate() {
                let mut new = words.clone();
                let _ = new.remove(idx);
                if (is_safe(&new)) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    println!("Part 2: {safe_count}");
}

fn is_safe(words: &Vec<&str>) -> bool {
    let mut safe_gap = true;
    let mut safe_order = false;
    let mut levels: Vec<i32> = Vec::new();

    for word in words {
        let num = word.parse::<i32>().unwrap();

        if let Some(&last) = levels.last() {
            if (num - last).abs() > 3 {
                safe_gap = false
            }
        }
        levels.push(num);
    }

    if (levels.is_sorted_by(|a, b| a < b)) || (levels.is_sorted_by(|a, b| a > b)) {
        safe_order = true;
    }
    return safe_gap && safe_order;
}

fn print_vec(vec: &Vec<i32>) {
    for v in vec {
        print!("{v} ")
    }
    println!()
}

fn print_words(vec: &Vec<&str>) {
    for v in vec {
        print!("{v} ")
    }
    println!()
}