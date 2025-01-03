use std::collections::HashMap;
use crate::utils::{read_lines, print_vec};

pub fn solve()
{
    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();

    let lines = read_lines("./input_files/day5_test_input.txt").unwrap();

    let mut is_instruction = true;

    for line in lines.into_iter() {
        let temp_line = line.unwrap();

        if temp_line.is_empty() {
            is_instruction = false;
            continue;
        }

        if is_instruction {
            let (index, value) = read_rule(&temp_line);
            rules.entry(index).or_default().push(value);
        
        } else {
            read_update(&mut updates, &temp_line);
        }
    }
    solve_p1(&updates, &rules);
    solve_p2(&mut updates, &rules);
}

fn solve_p2(updates: &mut Vec<Vec<usize>>, rules: &HashMap<usize, Vec<usize>>)
{
    let mut sum = 0;

    for update in updates.iter_mut() {
        if !check_update(&update, &rules) {
            reorder(update, &rules);
            sum += get_middle(update);
        }
    }

    println!("Part 2: {}", sum);
}

fn reorder<'a>(update: &'a mut Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> &'a Vec<usize>
{
    let len = update.len();
    for idx in 0..len {
        let num = update[idx];
        let update_rules: Option<&Vec<usize>> = rules.get(&num);
        if update_rules == None { 
            continue; 
        }
        for rule in update_rules.unwrap().iter() {
            //bubble down the invalid value until its valid
            while !validate_entry(*rule, idx, update) {
                update.swap(idx, idx-1);
            };
        }
    }

    return update;
}

fn solve_p1(updates: &Vec<Vec<usize>>, rules: &HashMap<usize, Vec<usize>>) 
{
    let mut sum = 0;

    for update in updates {
        if check_update(&update, &rules) {
            sum += get_middle(&update);
        }
    }

    println!("Part 1: {}", sum);
}



fn check_update(update: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool 
{
    for (idx, num) in update.iter().enumerate() {
        let update_rules: Option<&Vec<usize>> = rules.get(&num);
        if update_rules == None { 
            continue; 
        }
        for rule in update_rules.unwrap().iter() {
            if !validate_entry(*rule, idx, update) {
                return false;
            };
        }
    }
    return true;
}

fn validate_entry(rule_val: usize, position: usize, update: &Vec<usize>) -> bool
{
    for (i, num ) in update.into_iter().enumerate() {
        if *num == rule_val {
            if i < position {
                return false;
            }
        }
    }
    return true;
}

fn get_middle(update: &Vec<usize>) -> usize
{
    return update[update.len()/2]
}

fn read_update(updates: &mut Vec<Vec<usize>>, line: &String) 
{
    let mut v:Vec<usize> = Vec::new();

    for number in line.split(',') {
        let num = number.parse::<usize>().unwrap();
        v.push(num);
    }
    updates.push(v);
}

fn read_rule(line: &String) -> (usize, usize) 
{
    let mut index = 0;
    let mut value = 0;

    for (i, number) in line.split('|').enumerate() {
        match i {
            0 => index = number.parse::<usize>().unwrap(),
            1 => value = number.parse::<usize>().unwrap(),
            _ => break,
        }
    }
    return (index, value)
}

