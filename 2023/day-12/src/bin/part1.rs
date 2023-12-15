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
        .map(|(_rowidx, row)| {
            let _correct_row = row
                .group_lengths
                .iter()
                .map(|length| "#".repeat(*length as usize))
                .collect_vec();

            let _unknown_locations = row
                .springs
                .char_indices()
                .filter_map(|(char_idx, char)| match char == '?' {
                    true => Some(char_idx),
                    false => None,
                })
                .collect_vec();
            let _springs_total = row.group_lengths.iter().sum::<u32>();
            let _existing_springs = match row.springs.chars().counts().get(&'#') {
                Some(num) => *num,
                None => 0,
            };

            0
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
