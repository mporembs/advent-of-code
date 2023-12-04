fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut result = 0;
    let lines_vec: Vec<&str> = input.lines().map(|line| line.trim()).collect();

    let symbols = input.lines().map(|line| {
        line.trim()
            .match_indices(|character: char| {
                if character == '.' {
                    return false;
                } else if character.is_alphanumeric() {
                    return false;
                }
                true
            })
            .collect::<Vec<_>>()
    });

    for (line_idx, line) in symbols.enumerate() {
        let mut idx: usize = 0;

        for symbol in &line {
            let left_number: u32;
            if idx == 0 {
                left_number = lines_vec[line_idx]
                    .get(0..symbol.0)
                    .expect("to be a string")
                    .split(pat)
                    .chars()
                    .filter_map(|character| character.to_digit(10))
                    .map(|digit| digit.to_string())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap_or_default();
            } else {
                left_number = lines_vec[line_idx]
                    .get(line[idx - 1].0..symbol.0)
                    .expect("to be a string")
                    .chars()
                    .filter_map(|character| character.to_digit(10))
                    .map(|digit| digit.to_string())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap_or_default();
            }
            result += left_number;
            println!("Line {line_idx}: {left_number} before {}", symbol.1);
            idx += 1;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..",
        );
        assert_eq!(result, "4361".to_string());
    }
}
