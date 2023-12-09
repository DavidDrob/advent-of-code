use std::fs;
use std::collections::HashMap;

fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(mut lines: Vec<String>) {
    let mut directions = lines.first().cloned().unwrap();
    lines.drain(..2);

    let mut data: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in &lines {
        let line: Vec<&str> = line.split(" = ").collect();

        let mut iter = line[1].char_indices();
        let (start, _) = iter.nth(1).unwrap();
        let (end, _) = iter.nth(7).unwrap();
        let slice = &line[1][start..end];
        let destinations = slice.split(", ").collect::<Vec<_>>(); 

        data.insert(line[0], (destinations[0], destinations[1]));
    }

    let mut current = "AAA";
    let mut i = 0;

    while current != "ZZZ" {
        let id = i % &directions.len();
        let entry = data.get(current).unwrap();

        match &directions.chars().nth(id) {
            Some('L') => current = entry.0,
            Some('R') => current = entry.1,
            _ => break
        }
        i += 1;
    }

    println!("{i}");
}

fn part2(mut lines: Vec<String>) {
    let mut directions = lines.first().cloned().unwrap();
    lines.drain(..2);

    // let mut data: HashMap<(&str, char), (&str, &str)> = HashMap::new();
    let mut data: HashMap<&str, (char, &str, &str)> = HashMap::new();

    for line in &lines {
        let line: Vec<&str> = line.split(" = ").collect();

        let mut iter = line[1].char_indices();
        let (start, _) = iter.nth(1).unwrap();
        let (end, _) = iter.nth(7).unwrap();
        let slice = &line[1][start..end];
        let destinations = slice.split(", ").collect::<Vec<_>>(); 

        data.insert(line[0], (line[0].chars().nth(2).unwrap(), destinations[0], destinations[1]));
    }

    let start = data.iter().filter(|(k, _)| k.chars().nth(2).unwrap() == 'A').map(|(_, value)| *value).collect::<Vec<_>>();

    let mut i = 1;
    let mut next: Vec<&str> = Vec::new();
    let mut next_tmp: Vec<&str> = vec!();

    for entry in &start {
        match &directions.chars().nth(0) {
            Some('L') => next.push(entry.1),
            Some('R') => next.push(entry.2),
            _ => break
        }
    }

    while is_end(next.clone()) {
        for entry in &next {
            let paths = data.get(entry).unwrap();
            let id = i % &directions.len();

            match &directions.chars().nth(id) {
                Some('L') => next_tmp.push(paths.1),
                Some('R') => next_tmp.push(paths.2),
                _ => break
            };
        }
        i += 1;

        next = vec!();
        for x in &next_tmp {
            next.push(x);
        }
        next_tmp = vec!();
    }

    println!("{i}");
}

fn is_end(strings: Vec<&str>) -> bool {
    for s in strings {
        if s.chars().nth(2).unwrap() != 'Z' {
            return true;
        }
    }
    false
}