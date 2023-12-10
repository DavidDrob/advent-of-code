use std::fs;
use std::collections::HashMap;

fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(mut lines: Vec<String>) {
    let directions = lines.first().cloned().unwrap();
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
    let directions = lines.first().cloned().unwrap();
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

    let start = data.iter().filter(|(k, _)| k.chars().nth(2).unwrap() == 'A').map(|(k, _)| *k).collect::<Vec<_>>();

    let mut steps: Vec<usize> = vec!();

    for n in start {
        let mut current = n;
        let mut i = 0;

        while current.chars().nth(2).unwrap() != 'Z' {
            let id = i % &directions.len();
            let nodes = data.get(current).unwrap();

            match &directions.chars().nth(id) {
                Some('L') => current = nodes.0,
                Some('R') => current = nodes.1,
                _ => break
            }
            i += 1;
        }
        steps.push(i);
    }

    println!("{:?}", lcm_of_n(&steps));
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

fn lcm_of_n(numbers: &[usize]) -> usize {
    let mut result = numbers[0];
    for &num in numbers.iter().skip(1) {
        result = lcm(result, num);
    }
    result
}