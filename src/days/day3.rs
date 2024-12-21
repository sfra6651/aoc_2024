use std::fs;
use regex::{Match, Regex};

pub fn solve() 
{
    let contents = fs::read_to_string("./input_files/day3_input.txt").unwrap();
    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String) 
{
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let re_num = Regex::new(r"[0-9]+").unwrap();

    let caps = re.find_iter(&contents);

    let mut sum = 0;

    for cap in caps {
        sum += solve_instruction(&cap.as_str(), &re_num);
    }

    println!("Part 1: {}", sum);
}

fn part2(contents: &String) 
{
    let re_num = Regex::new(r"[0-9]+").unwrap();
    let re_mul = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let regex = vec![&re_mul, &re_do, &re_dont];

    let mut next = find_next_instruction(contents, &regex);
    let mut compute = true;
    let mut sum = 0;
    let mut index = 0;

    while next.is_some(){
        let m = next.unwrap();
        let instruction = m.as_str();

        if re_do.is_match(&instruction) {
            compute = true;
        } else if  re_dont.is_match(&instruction) {
            compute = false;
        } else if re_mul.is_match(&instruction) && compute {
            sum += solve_instruction(&instruction, &re_num);
        }

        index += m.end();
        next = find_next_instruction(&contents[index..], &regex)
    }

    println!("Part 2: {}", sum);

}

fn find_next_instruction<'a>(contents: &'a str, regex: &Vec<&Regex>) -> Option<Match<'a>>
{
    let matches = vec![
        regex[0].find(&contents), 
        regex[1].find(&contents), 
        regex[2].find(&contents),
        ];

    let first_match = matches
        .into_iter()
        .filter(|mtch| mtch.is_some())
        .map(|mtch| mtch.unwrap())
        .min_by_key(|x| x.start());


    return first_match;

}

fn solve_instruction(instruction: &str, re: &Regex) -> i32 
{
    let caps = re.find_iter(&instruction);
    let (mut x,mut y) = (0, 0);

    for (idx, cap) in caps.enumerate() {
        match idx {
            0 => x = cap.as_str().parse().unwrap(),
            1 => y = cap.as_str().parse().unwrap(),
            _ => break,
        }
    }
    return x * y;
}