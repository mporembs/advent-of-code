use std::{char, collections::HashMap, string};

use itertools::Itertools;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut split_input = input.lines().filter(|line| !line.eq(&""));
    let mut instructions = split_input
        .next()
        .unwrap()
        .chars()
        .map(|x| if x.eq(&'L') { 0 } else { 1 })
        .cycle();

    let map: HashMap<String, [String; 2]> = HashMap::from_iter(split_input.map(|line| {
        let (location, options) = line.split_once(" = ").unwrap();
        let (l, r) = options.split_once(", ").unwrap();
        let l = l.replace('(', "");
        let r = r.replace(')', "");

        (location.to_string(), [l.to_string(), r.to_string()])
    }));

    let mut positions = map
        .iter()
        .filter_map(|(key, _)| {
            if key.ends_with('A') {
                Some((key.as_str(), 0usize))
            } else {
                None
            }
        })
        .collect_vec();

    let pos_len = positions.len();
    let mut finished_len = 0;

    let mut steps = || -> usize {
        loop {
            let next_instruction = instructions.next().unwrap();

            for p in positions.iter_mut().filter(|(pos, _)| !pos.ends_with("Z")) {
                p.0 = &map[p.0][next_instruction];
                if p.0.ends_with("Z") {
                    finished_len += 1;
                }
                p.1 += 1;
            }

            if finished_len == pos_len {
                dbg!(&positions);
                return positions
                    .iter()
                    .fold(1, |acc, (_, steps)| num::integer::lcm(acc, *steps));
            }
        }
    };
    steps().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, "6".to_string());
    }
}
