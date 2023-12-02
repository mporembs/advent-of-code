fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    //let result = dbg!(input.split("/n").collect());
    let result = input
        .lines()
        .map(|line| {
            let mut char_it = line.chars().filter_map(|character| character.to_digit(10));
            let first = char_it.next().expect("Should be a valid number.");
            match char_it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("Should be a number.")
        })
        .sum::<u32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        );
        assert_eq!(result, "142".to_string());
    }
}
