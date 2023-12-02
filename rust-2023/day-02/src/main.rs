use std::fs;
use std::collections::HashMap;

fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let rules = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let mut invalid = 0;

    for (id, line) in lines.iter().enumerate() {
        let splitted: Vec<&str> = line.split(":").collect();
        let set: Vec<&str> = splitted[1].split(";").collect();

        for subset in set {
            let subset: Vec<&str> = subset.split(", ").collect();

            let mut found = false;

            for (i, element) in subset.iter().enumerate() {
                let mut element_data: Vec<&str> = element.split(" ").collect();

                if i == 0 {
                    element_data.remove(0);
                }

                if element_data[0].parse::<u32>().unwrap() > rules[element_data[1]] {
                    invalid += id+1;
                    found = true;
                    break;
                }

            }
            if found {
                break;
            }

        }
    }

    let games_sum: usize = (1..lines.len()+1).collect::<Vec<usize>>().iter().sum();
    println!("{}", games_sum - invalid);
}


fn part2(lines: Vec<String>) {
    let mut result = 0;

    for (id, line) in lines.iter().enumerate() {
        let splitted: Vec<&str> = line.split(":").collect();
        let set: Vec<&str> = splitted[1].split(";").collect();

        let mut fewest = HashMap::from([
            ("red", 1),
            ("green", 1),
            ("blue", 1)
        ]);

        for subset in set {
            let subset: Vec<&str> = subset.split(", ").collect();

            for (i, element) in subset.iter().enumerate() {
                let mut element_data: Vec<&str> = element.split(" ").collect();

                if i == 0 {
                    element_data.remove(0);
                }
                if element_data[0].parse::<u32>().unwrap() > fewest[element_data[1]] {
                    fewest.insert(element_data[1], element_data[0].parse::<u32>().unwrap());
                }
            }
        }

        let product: u32 = fewest.into_iter().map(|(_c, a)| a).product();
        result += product;
    }

    println!("{:?}", result);
}