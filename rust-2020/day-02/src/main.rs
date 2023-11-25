use std::fs;

fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut valid_passwords = 0;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if let [range, key, password] = parts.as_slice() {
            let range: Vec<usize> = range.split("-").map(|s| s.parse().unwrap()).collect();
            let (min, max) = (range[0], range[1]);
            let key = key.chars().next().unwrap();

            let occurrences = password.chars()
                .filter(|&c| c == key)
                .count();

            if occurrences >= min && occurrences <= max {
                valid_passwords += 1;
            }
        }
    }

    println!("{:?}", valid_passwords);
}

fn part2(lines: &Vec<String>) {
    let mut valid_passwords = 0;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if let [range, key, password] = parts.as_slice() {
            let range: Vec<usize> = range.split("-").map(|s| s.parse().unwrap()).collect();
            let (p1, p2) = (range[0] - 1, range[1] - 1);
            let key = key.chars().next().unwrap();

            let password_vec: Vec<char> = password.chars().collect();

            if (password_vec[p1] == key) ^ (password_vec[p2] == key) {
                valid_passwords += 1;
            }
        }
    }

    println!("{:?}", valid_passwords);
}
