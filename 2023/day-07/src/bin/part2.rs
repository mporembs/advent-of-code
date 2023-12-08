use std::collections::{BTreeMap, HashMap};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
enum HandType {
    FiveOfaKind = 7,
    FourOfaKind = 6,
    FullHouse = 5,
    ThreeOfaKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Hand<'a> {
    bid: u32,
    cards: &'a str,
}

fn part2(input: &str) -> String {
    let mut scored_hands: BTreeMap<String, u32> = BTreeMap::new();
    let card_values = HashMap::from([
        ('A', "z"),
        ('K', "y"),
        ('Q', "x"),
        ('J', "n"),
        ('T', "w"),
        ('9', "v"),
        ('8', "u"),
        ('7', "t"),
        ('6', "s"),
        ('5', "r"),
        ('4', "q"),
        ('3', "p"),
        ('2', "o"),
    ]);
    let hands = input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(' ');
            Hand {
                cards: line_iter.next().expect("a string of cards."),
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

    hands.iter().for_each(|hand| {
        let mut score_vec: Vec<&str> = Vec::new();
        match hand_type(hand.cards) {
            HandType::FiveOfaKind => score_vec.push("7"),
            HandType::FourOfaKind => score_vec.push("6"),
            HandType::FullHouse => score_vec.push("5"),
            HandType::ThreeOfaKind => score_vec.push("4"),
            HandType::TwoPair => score_vec.push("3"),
            HandType::OnePair => score_vec.push("2"),
            HandType::HighCard => score_vec.push("1"),
        };
        hand.cards
            .chars()
            .for_each(|character| score_vec.push(*card_values.get(&character).unwrap()));
        scored_hands.insert(score_vec.concat(), hand.bid);
    });
    // dbg!(&scored_hands);

    let total = scored_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, hand)| acc + ((idx as u32 + 1) * hand.1));

    total.to_string()
}

fn hand_type(card: &str) -> HandType {
    let mut card_map: BTreeMap<char, u32> = BTreeMap::new();
    card.chars().for_each(|character| {
        card_map
            .entry(character)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });
    let joker_count = card_map.remove(&'J').unwrap_or(0);

    if joker_count == 5 {
        return HandType::FiveOfaKind;
    };

    let mut count_vec: Vec<(&char, &u32)> = card_map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    let mut count_iter = count_vec.iter();

    match (count_iter.next().unwrap().1) + joker_count {
        5 => HandType::FiveOfaKind,
        4 => HandType::FourOfaKind,
        3 => match count_iter.next().unwrap().1 {
            2 => HandType::FullHouse,
            _ => HandType::ThreeOfaKind,
        },
        2 => match count_iter.next().unwrap().1 {
            2 => HandType::TwoPair,
            1 => HandType::OnePair,
            _ => unreachable!(),
        },
        1 => HandType::HighCard,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, "5905".to_string());
    }
}
