use std::fs;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    Pair,
    High
}

fn main () {
    let content = fs::read_to_string("./input.txt").expect("cant open file!");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    part1(lines.clone());
    // part2(lines);
}

fn part1(lines: Vec<String>) {
    let mut hands: Vec<(&str, Type, usize)> = vec!();
    let mut result = 0;

    for line in &lines {
        let line = line.split(" ").collect::<Vec<&str>>();
        hands.push((line[0], determine_hand(line[0]), line[1].parse::<usize>().unwrap()));
    }

    let types: Vec<Type> = Vec::from([Type::Five, Type::Four, Type::FullHouse, Type::Three, Type::TwoPair, Type::Pair, Type::High]);

    let mut index = 1;
    for hand_type in types.iter().rev() {

        let mut hands_of_type: Vec<(&str, Type, usize)> = hands.iter().filter(|(_, v, _)| v == hand_type).cloned().collect();
        hands_of_type.sort_by(|a, b| compare(b.0, a.0));

        for hand in hands_of_type {
            result += hand.2 * index;
            index += 1;
        }
    }

    println!("{:?}", result);
}

fn compare(a: &str, b: &str) -> Ordering {
    let mut a_chars = a.chars();
    let mut b_chars = b.chars();

    let mut order: Ordering = Ordering::Equal;
    while order == Ordering::Equal {
        let ac = a_chars.next().unwrap();
        let bc = b_chars.next().unwrap();
        order = find_larger(ac, bc);
    }

    return order;
}

fn find_larger(a: char, b: char) -> Ordering {
    let values = "AKQJT987654321";

    let a_index = values.find(a);
    let b_index = values.find(b);

    return a_index.cmp(&b_index);
}

fn determine_hand(cards: &str) -> Type {
    for card in cards.chars().take(4) {
        if cards.match_indices(card).collect::<Vec<_>>().len() == 5 {
            return Type::Five;
        }
        else if cards.match_indices(card).collect::<Vec<_>>().len() == 4 {
            return Type::Four;
        }
        else if cards.match_indices(card).collect::<Vec<_>>().len() == 3 {
            let rest = cards.chars().filter(|&c| c != card).collect::<Vec<_>>();

            if rest[0] == rest[1] {
                return Type::FullHouse;
            }
            return Type::Three;
        }
        else if cards.match_indices(card).collect::<Vec<_>>().len() == 2 {
            let rest = cards.chars().filter(|&c| c != card).collect::<Vec<_>>();
            let mut seen = vec!();

            if rest[0] == rest[1] && rest[1] == rest[2] {
                return Type::FullHouse;
            }

            for c in rest {
                if seen.contains(&c) {
                    return Type::TwoPair;
                }
                seen.push(c);
            }

            return Type::Pair;
        }
    }
    return Type::High;
}