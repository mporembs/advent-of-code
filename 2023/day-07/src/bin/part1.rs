use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

// #[derive(Debug)]
// enum HandType {
//     FiveOfaKind = 1,
//     FourOfaKind = 2,
//     FullHouse = 3,
//     ThreeOfaKind = 4,
//     TwoPair = 5,
//     OnePair = 6,
//     HighCard = 7,
//     None = 8,
// }

#[derive(Debug)]
struct Hand {
    bid: u32,
    cards: Vec<char>,
}

fn part1(input: &str) -> String {
    let card_values = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);

    let hands = input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(' ');
            Hand {
                cards: line_iter
                    .next()
                    .expect("a string of cards.")
                    .chars()
                    .collect::<Vec<char>>(),
                bid: line_iter
                    .next()
                    .expect("a string slice of numeric chars")
                    .chars()
                    .filter_map(|time_char| match time_char.to_digit(10) {
                        Some(num) => Some(num.to_string()),
                        None => None,
                    })
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("a single number"),
            }
        })
        .collect::<Vec<Hand>>();
    dbg!(hands);
    "0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, "288".to_string());
    }
}
