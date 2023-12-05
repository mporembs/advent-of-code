use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}
#[derive(Debug, Hash, PartialEq, Eq)]
struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

fn part2(input: &str) -> String {
    let cards: Vec<Card> = input
        .lines()
        .map(|line| {
            let mut filtered_line = line
                .trim()
                .split(&[':', '|'])
                .map(|sub_sting| sub_sting.trim());
            Card {
                number: filtered_line
                    .next()
                    .expect("to be a valid string")
                    .chars()
                    .filter(|character| character.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("a valid number"),
                winning_numbers: filtered_line
                    .next()
                    .expect("to be a valid string")
                    .split(' ')
                    .filter_map(|sub| match sub.trim().parse::<u32>() {
                        Ok(value) => Some(value),
                        Err(_) => None,
                    })
                    .collect::<Vec<u32>>(),
                drawn_numbers: filtered_line
                    .next()
                    .expect("to be a string")
                    .split(' ')
                    .filter_map(|sub| match sub.trim().parse::<u32>() {
                        Ok(value) => Some(value),
                        Err(_) => None,
                    })
                    .collect::<Vec<u32>>(),
            }
        })
        .collect();

    let mut card_counts: HashMap<u32, u32> = HashMap::new();

    for card in &cards {
        card_counts.insert(card.number, 1);
    }

    cards.iter().for_each(|card| {
        let mut card_score = 0;
        for winner in &card.winning_numbers {
            match card
                .drawn_numbers
                .iter()
                .any(|drawn_num| drawn_num == winner)
            {
                true => card_score += 1,
                false => continue,
            }
        }
        for number in 1..card_score + 1 {
            let mod_card_number = card.number + number;
            let current_copies_of_card = &card_counts
                .get(&card.number)
                .expect("to be a number")
                .to_owned();
            match mod_card_number <= cards.len() as u32 {
                true => {
                    // println!(
                    //     "Card {:?} wins {} copies of card {}",
                    //     card.number,
                    //     1 * current_copies_of_card,
                    //     mod_card_number
                    // );
                    card_counts
                        .entry(card.number + number)
                        .and_modify(|count| *count += 1 * current_copies_of_card);
                }
                false => continue,
            }
        }
    });

    let total_card: u32 = card_counts
        .values()
        .cloned()
        .map(|card_total| card_total)
        .sum();
    total_card.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "30".to_string());
    }
}
