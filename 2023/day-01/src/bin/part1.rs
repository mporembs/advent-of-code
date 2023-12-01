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
            let mut final_string = String::from("");
            let sub_string = line.trim_start().trim_matches(char::is_alphabetic);
            final_string.push(sub_string.char_indices().nth(0).unwrap().1);
            final_string.push(sub_string.char_indices().nth_back(0).unwrap().1);
            final_string.parse::<u32>().unwrap()
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
