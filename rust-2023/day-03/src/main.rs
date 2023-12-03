use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let mut results: HashSet<(usize, usize, usize)> = HashSet::new();
    let numbers = get_numbers(&lines);
    
    for (i, line) in lines.iter().enumerate() {
        let pattern = Regex::new(r"[^0-9.]").unwrap();
        let matches: Vec<usize> = pattern.find_iter(line).map(|m| m.start()).collect();
        
        for symbol in &matches {
            for row in i-1..i+2 {
                for col in symbol-1..symbol+2 {
                    for (key, _value) in numbers.iter() {
                        if key.0 == row && ((key.1 == col) || (key.2 == col)) {
                            results.insert(*key);
                        }
                    }
                }
            }
        }
    }

    let result: usize = results.iter().map(|k| numbers[k]).sum();
    println!("{:?}", result);
}

fn part2(lines: Vec<String>) {
    let mut result: usize = 0;
    let numbers = get_numbers(&lines);
    
    for (i, line) in lines.iter().enumerate() {
        let asterisks: Vec<usize> = line.match_indices("*").map(|(i, _)| i).collect(); 

        for asterisk in &asterisks {
            let mut adjacent_numbers: HashSet<((usize, usize, usize), &usize)> = HashSet::new();

            for row in i-1..i+2 {
                for col in asterisk-1..asterisk+2 {
                    for (key, value) in numbers.iter() {
                        if key.0 == row && ((key.1 == col) || (key.2 == col)) {
                            adjacent_numbers.insert(((key.0, key.1, key.2), value));
                        }
                    }
                }
            }

            if adjacent_numbers.len() == 2 {
                result += adjacent_numbers.iter().map(|a| a.1).product::<usize>();
            }
        }
    }

    println!("{:?}", result);
}


fn get_numbers(lines: &Vec<String>) -> HashMap<(usize, usize, usize), usize> {
    let mut numbers: HashMap<(usize, usize, usize), usize> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let pattern = Regex::new(r"\d+\d*").unwrap();
        let matches: Vec<(usize, usize)> = pattern.find_iter(line).map(|m| (m.start(), m.end()-1)).collect();

        for range in matches {
            numbers.insert((i, range.0, range.1), line.as_str()[range.0..range.1+1].parse::<usize>().unwrap());
        }
    }

    numbers
}