use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let nice = input
        .lines()
        .filter(|&line| {
            if !adequate_vowels(line) {
                return false;
            }
            if !has_double_char(line) {
                return false;
            }
            if has_forbidden(line) {
                return false;
            }
            true
        })
        .count();
    nice.to_string()
}

fn has_forbidden(input: &str) -> bool {
    let mut forbidden = ["ab", "cd", "pq", "xy"].iter();
    while let Some(forb) = forbidden.next() {
        match input.find(forb) {
            Some(_) => return true,
            None => (),
        }
    }
    false
}

fn has_double_char(input: &str) -> bool {
    let mut chars_iter = input.chars().tuple_windows();
    while let Some((ch_a, ch_b)) = chars_iter.next() {
        if ch_a.eq(&ch_b) {
            return true;
        }
    }
    false
}

fn adequate_vowels(input: &str) -> bool {
    // dbg!(input);
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut count = 0;
    let mut chars_iter = input.chars();
    while let Some(ch) = chars_iter.next() {
        match vowels.contains(&ch) {
            true => count += 1,
            false => (),
        }
        if count > 2 {
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
        let result = part1(
            "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb",
        );
        assert_eq!(result, "2".to_string());
    }
}
