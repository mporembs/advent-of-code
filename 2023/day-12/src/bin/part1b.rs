use itertools::Itertools;

struct Row<'a> {
    springs: &'a str,
    group_lengths: Vec<u32>,
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let fill_vec = vec![
        '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.',
        '#', '.',
    ];

    let spring_records = input
        .lines()
        .map(|line| {
            let split_line = line
                .split_ascii_whitespace()
                .collect_tuple::<(&str, &str)>()
                .unwrap();
            Row {
                springs: split_line.0,
                group_lengths: split_line
                    .1
                    .chars()
                    .filter_map(|character| character.to_digit(10))
                    .collect_vec(),
            }
        })
        .collect_vec();

    let total: u32 = spring_records
        .iter()
        .enumerate()
        .map(|(rowidx, row)| {
            let correct_row = row
                .group_lengths
                .iter()
                .map(|length| "#".repeat(*length as usize))
                .collect_vec();

            let unknown_locations = row
                .springs
                .char_indices()
                .filter_map(|(char_idx, char)| match char == '?' {
                    true => Some(char_idx),
                    false => None,
                })
                .collect_vec();

            let replacement_combos = &fill_vec[0..unknown_locations.len()]
                .iter()
                .combinations_with_replacement(unknown_locations.len())
                .unique()
                .collect_vec();
            println!(
                "Row:{} of Rep Combos: {}",
                rowidx + 1,
                replacement_combos.len()
            );
            let possible_solutions = replacement_combos
                .iter()
                .map(|rep_combo| {
                    let mut fill_iter = rep_combo.iter();
                    let mut new_row = row.springs.chars().collect_vec();
                    for i in &unknown_locations {
                        let new_char = fill_iter.next().unwrap();
                        new_row[*i] = **new_char;
                    }
                    new_row.iter().collect::<String>()
                })
                .collect_vec()
                .iter()
                .unique()
                .filter(|combo| {
                    let split_combo = combo
                        .split('.')
                        .filter_map(|split_str| match split_str == "" {
                            true => None,
                            false => Some(split_str.to_string()),
                        })
                        .collect_vec();
                    match split_combo.cmp(&correct_row) {
                        std::cmp::Ordering::Equal => true,
                        _ => false,
                    }
                })
                .count() as u32;

            possible_solutions
        })
        .sum();
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
####.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(result, "21".to_string());
    }
}
