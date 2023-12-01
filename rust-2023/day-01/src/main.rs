use std::fs;

fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: Vec<String>) {
    let numbers: Vec<char> = "123456789".chars().collect();
    let mut results: Vec<u32> = Vec::new();

    for line in lines {
        let mut a = '0';
        let mut b = '0';

        for character in line.chars() {
            if numbers.contains(&character) {
                a = character;
                break;
            }
        }

        for character in line.chars().rev() {
            if numbers.contains(&character) {
                b = character;
                break;
            }
        }

        let n: u32 = format!("{a}{b}").parse::<u32>().unwrap();
        results.push(n);
    }

    let sum: u32 = results.iter().sum();
    println!("{:?}", sum);
}

fn part2(lines: Vec<String>) {
    let numbers: Vec<&str> = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"].to_vec();
    let mut result: Vec<u32> = Vec::new();

    for string in lines {
        let mut found: Vec<&str> = vec![];

        for number in &numbers {
            let splitted: Vec<&str> = string.split(number).collect();

            // the number is a substring if it can split the string
            if splitted.len() > 1 {
                found.push(number);
            } 
        }

        let mut min = 99999;
        let mut min_n = "";
        let mut max = 0;
        let mut max_n = "";
        for number in found {
            let occurences: Vec<(usize, &str)> = string.match_indices(number).collect();
            for occurence in occurences {
                if occurence.0 < min {
                    min = occurence.0;
                    min_n = occurence.1;
                    if min_n.len() > 1 {
                        min_n = numbers[numbers.iter().position(|&r| r == min_n).unwrap() + 9];
                    }
                }

                if occurence.0 >= max {
                    max = occurence.0;
                    max_n = occurence.1;
                    if max_n.len() > 1 {
                        max_n = numbers[numbers.iter().position(|&r| r == max_n).unwrap() + 9];
                    }
                }
            }
            
        }

        let n: u32 = format!("{min_n}{max_n}").parse::<u32>().unwrap();
        result.push(n);
    }

    let sum: u32 = result.iter().sum();
    println!("{:?}", sum);
    
}
