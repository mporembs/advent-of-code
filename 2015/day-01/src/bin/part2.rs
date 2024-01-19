fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut basement_idx = 0;
    let mut satisfied = false;
    input.chars().enumerate().fold(0, |acc, (ch_idx, ch)| {
        let new = acc
            + match ch {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
        match (new, satisfied) {
            (-1, false) => {
                basement_idx = ch_idx + 1;
                satisfied = true;
            }
            _ => (),
        }
        new
    });
    basement_idx.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("()())");
        assert_eq!(result, "5".to_string());
    }
}
