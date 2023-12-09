use std::{char, collections::HashMap};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Branches {
    left: String,
    right: String,
}

fn part1(input: &str) -> String {
    let mut split_input = input.lines().filter(|line| !line.eq(&""));
    let route = split_input.next().unwrap().chars().collect::<Vec<char>>();

    let node_map: HashMap<&str, Branches> = split_input
        .clone()
        .map(|node_line| {
            let mut iter = node_line.split(' ');

            (
                iter.next().unwrap(),
                Branches {
                    left: iter
                        .nth(1)
                        .unwrap()
                        .chars()
                        .filter(|character| character.is_ascii_alphabetic())
                        .collect::<String>(),
                    right: iter
                        .next()
                        .unwrap()
                        .chars()
                        .filter(|character| character.is_ascii_alphabetic())
                        .collect::<String>(),
                },
            )
        })
        .collect();

    let mut current_node: &str = "AAA";
    let mut steps = 0;

    while !(current_node == "ZZZ") {
        for turn in &route {
            match turn {
                'L' => {
                    current_node = &node_map.get(current_node).unwrap().left;
                    steps += 1
                }
                'R' => {
                    current_node = &node_map.get(current_node).unwrap().right;
                    steps += 1
                }
                _ => break,
            }
        }
    }

    // dbg!(route);
    // dbg!(node_map);
    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "6".to_string());
    }
}
