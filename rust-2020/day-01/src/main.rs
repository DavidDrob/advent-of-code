use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let numbers: HashSet<u32> = content.lines()
        .map(|s| s.parse::<u32>())
        .filter_map(Result::ok)
        .into_iter()
        .collect();

    part1(numbers.clone());
    part2(numbers);
}

fn part1(numbers: HashSet<u32>)  {
    let mut result = 1;
    for n in &numbers {
        if numbers.contains(&(2020 - n)) {
            result *= n;
        }
    }

    println!("{:?}", result);
}

fn part2(numbers: HashSet<u32>)  {
    let mut sums = HashMap::new();

    for i in &numbers {
        for j in &numbers {
            let sum = i + j;
            sums.insert(sum, vec![i, j]);
        }
    }

    for (sum, vars) in sums {
        if sum <= 2020 && numbers.contains(&(2020 - sum)) {
            println!("{:?}", (2020 - sum) * vars[0] * vars[1] );
            break;
        }
    }
}
