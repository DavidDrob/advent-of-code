use std::fs;
use std::collections::HashMap;

fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let overflow = lines[0].len();

    let mut y = 0;
    let mut trees = 0;

    for line in lines {
        if (line.as_bytes()[y % overflow] as char) == '#' {
            trees += 1;
        }

        y += 3;
    }

    println!("{:?}", trees);
}

fn part2(lines: &Vec<String>) {
    let overflow = lines[0].len();

    let mut coords = HashMap::new();
    let mut r1d2 = 0;
    let slopes = vec![1, 3, 5, 7];

    for (i, line) in lines.iter().enumerate() {

        for y in &slopes {
            if (line.as_bytes()[(y * i) % overflow] as char) == '#' {
                let entry = coords.entry(*y).or_insert(0);
                *entry += 1;
            }
        }

        if i % 2 == 0 {
            if (line.as_bytes()[(i/2) % overflow] as char) == '#' {
                r1d2 += 1;
            }
        }

    }

    let mut total: u128 = 1;
    for (slope, occurences) in &coords {
        total *= occurences;
    }
    total *= r1d2;

    println!("{:?}", total);

}
