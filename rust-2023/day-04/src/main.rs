use std::fs;
use std::collections::HashMap;

fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let mut result = 0;
    for line in lines {
        let mut points = 0;

        let data = line.split(": ").collect::<Vec<&str>>()[1];
        let winning = data.split(" | ").collect::<Vec<&str>>()[0].split(" ").filter_map(|s| s.parse().ok()).collect::<Vec<usize>>();
        let numbers = data.split(" | ").collect::<Vec<&str>>()[1].split(" ").filter_map(|s| s.parse().ok()).collect::<Vec<usize>>();

        for number in numbers {
            if winning.contains(&number) {
                points += 1;
            }
        }

        if points > 2 {
            result += 2_u32.pow(points-1);
        } else {
            result += points;
        }
    }
    
    println!("{:?}", result);
}

fn part2(lines: Vec<String>) {
    let mut copies: HashMap<usize, usize> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let mut points = 0;

        let data = line.split(": ").collect::<Vec<&str>>()[1];
        let winning = data.split(" | ").collect::<Vec<&str>>()[0].split(" ").filter_map(|s| s.parse().ok()).collect::<Vec<usize>>();
        let numbers = data.split(" | ").collect::<Vec<&str>>()[1].split(" ").filter_map(|s| s.parse().ok()).collect::<Vec<usize>>();

        for number in numbers {
            if winning.contains(&number) {
                points += 1;
            }
        }

        match copies.get(&(i+1)) {
            Some(count) => { 
                for _j in 0..*count {
                    for p in i+1..=points+i {
                        match copies.get(&(p+1)) {
                            Some(count) => { copies.insert(p+1, count+1); }
                            None => { copies.insert(p+1, 1); }
                        }
                    }
                }
                match copies.get(&(i+1)) {
                    Some(count) => { copies.insert(i+1, count+1); }
                    None => { copies.insert(i+1, 1); }
                }
             }
            None => { copies.insert(i+1, 1); }
        }

        // handle originals
        for p in i+1..=points+i {
            match copies.get(&(p+1)) {
                Some(count) => { copies.insert(p+1, count+1); }
                None => { copies.insert(p+1, 1); }
            }
        }
    }

    let sum: usize = copies.iter().map(|(_i, n)| n).sum();
    println!("{:?}", sum);
}
