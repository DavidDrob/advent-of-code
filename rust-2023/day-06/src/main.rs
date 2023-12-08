use std::fs;

fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let first_line: Vec<&str> = lines[0].split(":     ").collect::<Vec<&str>>()[1].split("  ").collect();
    let times: Vec<usize> = first_line.iter().filter_map(|s| s.trim().parse::<usize>().ok()).collect();

    let second_line: Vec<&str> = lines[1].split(": ").collect::<Vec<&str>>()[1].split("  ").collect();
    let distances: Vec<usize> = second_line.iter().filter_map(|s| s.trim().parse::<usize>().ok()).collect();

    let mut result = 1;

    for (i, time) in times.iter().enumerate() {
        let range = (TryInto::<usize>::try_into(1).unwrap())..=*time;
        let mut win_sum = 0;

        for t in range.clone().into_iter() {
            if (range.end() - t) * t > distances[i] {
                win_sum += 1;
            }
        }
        result *= win_sum;
    }

    println!("{:?}", result);
}

fn part2(lines: Vec<String>) {
    let first_line: Vec<&str> = lines[0].split(":     ").collect::<Vec<&str>>()[1].split("  ").collect();
    let time: usize = first_line 
        .into_iter()
        .map(|s| s.trim())
        .fold(String::new(), |a, b| a + b)
        .parse().unwrap();

    let second_line: Vec<&str> = lines[1].split(": ").collect::<Vec<&str>>()[1].split("  ").collect();
    let distance: usize = second_line
        .into_iter()
        .map(|s| s.trim())
        .fold(String::new(), |a, b| a + b)
        .parse().unwrap();

    let range = (TryInto::<usize>::try_into(1).unwrap())..=time;
    let mut win_sum = 0;

    for t in range.clone().into_iter() {
        if (range.end() - t) * t > distance {
            win_sum += 1;
        }
    }

    println!("{:?}", win_sum);
}
