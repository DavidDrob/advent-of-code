use std::fs;
use std::collections::HashMap;
 
fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let strings: &Vec<String> = &content.lines().map(|s| s.split(",").map(|s| s.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>()[0];
 
    part1(strings);
    part2(strings);
}
 
 
fn part1(strings: &Vec<String>) {
    let mut sum: u32 = 0;
 
    for string in strings {
        let hash = hash(string);
        sum = sum + hash as u32;
    }
 
    println!("{:?}", sum);
}
 
fn part2(strings: &Vec<String>) {
    let mut boxes: HashMap<u8, Vec<(String, u8)>> = HashMap::new();
 
    for string in strings {
        if string.contains("=") {
            let data: Vec<&str> = string.split("=").collect();
            let label = &String::from(data[0]);
 
            if boxes.contains_key(&hash(label)) {
                let lenses: &mut Vec<(String, u8)> = boxes.get_mut(&hash(label)).unwrap();
 
                let exists = lenses.into_iter().any(|(k, _)| *k == String::from(data[0]));
 
                if exists {
                    lenses.iter_mut().for_each(|(k, v)| {
                        if *k == String::from(data[0]) {
                            *v = data[1].parse::<u8>().unwrap();
                        }
                    });
                } else {
                    lenses.push((label.to_string(), data[1].parse::<u8>().unwrap()))
                }
            } else {
                boxes.insert(hash(label), Vec::from([(String::from(data[0]), data[1].parse::<u8>().unwrap())]));
            }
        } else {
            let label: String = string.chars().take(string.len() - 1).collect();
 
            match boxes.get_mut(&hash(&label)) {
                Some(lenses) => lenses.retain(|(k, _)| *k != label),
                None => (),
            }
        }
    }
 
    let mut focusing_power: u128 = 0;
 
    for lens_box in &boxes {
        for (i, lens) in lens_box.1.iter().enumerate() {
            focusing_power = focusing_power + ((*lens_box.0 as u16 + 1) as u128 * (i + 1) as u128 * lens.1 as u128);
        }
    }
    println!("{:?}", focusing_power);
}
 
fn hash(string: &String) -> u8 {
    let mut hash: u16 = 0;
    let chars: Vec<char> = string.chars().collect();
 
    for c in chars {
        hash = hash + c as u16;
        hash = hash * 17;
        hash = hash % 256;
    }
 
    hash as u8
}