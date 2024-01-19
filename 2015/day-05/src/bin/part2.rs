use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let nice = input
        .lines()
        .filter(|&line| {
            if !has_repeated_pair(line) {
                return false;
            }
            if !has_seperated_pair(line) {
                return false;
            }
            true
        })
        .count();
    nice.to_string()
}

fn has_repeated_pair(input: &str) -> bool {
    let mut pairs_iter = input.chars().enumerate().filter_map(|(ch_idx, _ch)| {
        if ch_idx < input.len() - 1 {
            Some(&input[ch_idx..=ch_idx + 1])
        } else {
            None
        }
    });
    while let Some(pair) = pairs_iter.next() {
        if input.matches(pair).collect_vec().len() > 1 {
            return true;
        }
    }
    false
}

fn has_seperated_pair(input: &str) -> bool {
    let mut set_iter = input.chars().tuple_windows::<(_, _, _)>();

    while let Some((ch_a, _ch_b, ch_c)) = set_iter.next() {
        if ch_a.eq(&ch_c) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy",
        );
        assert_eq!(result, "2".to_string());
    }
}
