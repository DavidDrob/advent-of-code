use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let lines: VecDeque<String> = content.lines().map(String::from).collect();

    part1(lines.clone());
}

fn part1(mut lines: VecDeque<String>) {
    let mut seeds = lines.front().map(|a| a.split(" ").filter_map(|s| s.parse::<i64>().ok()).collect::<Vec<i64>>()).unwrap_or_default();
    lines.drain(..2);

    let mut maps: Vec<(i64, i64, i64)> = vec!();

    for line in lines {
        if line.contains(":") {
            maps = vec!();
        }
        else if line == "" {
            let mut updated: Vec<usize> = vec!();

            for map in &maps {
                for (i, seed) in seeds.iter_mut().enumerate() {
                    if *seed >= map.1 && *seed < map.1 + map.2 && !updated.contains(&i) { // carefull with this logic
                        *seed += map.0-map.1;
                        updated.push(i);
                    }
                }
            }

        }
        else {
            let map_ints = line.split(" ").filter_map(|s| s.parse::<i64>().ok()).collect::<Vec<i64>>();
            let map_tuple: (i64, i64, i64) = (map_ints[0], map_ints[1], map_ints[2]); 
            maps.push(map_tuple);
        }

    }

    let min = seeds.iter().min().unwrap();
    println!("{:?}", min);
}