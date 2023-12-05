fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}
#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

fn part1(input: &str) -> String {
    let card: Vec<_> = input
        .lines()
        .map(|line| {
            let mut filtered_line = line
                .trim()
                .split(&[':', '|'])
                .map(|sub_sting| sub_sting.trim())
                .filter(|sub_string| !sub_string.contains("Card"));
            Card {
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

    let total = card
        .iter()
        .map(|game| {
            let mut game_score = 0;
            for winner in &game.winning_numbers {
                match game
                    .drawn_numbers
                    .iter()
                    .any(|drawn_num| drawn_num == winner)
                {
                    true => {
                        if game_score == 0 {
                            game_score += 1
                        } else {
                            game_score = game_score * 2
                        }
                    }
                    false => continue,
                }
            }
            game_score
        })
        .sum::<u32>();

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "13".to_string());
    }
}
