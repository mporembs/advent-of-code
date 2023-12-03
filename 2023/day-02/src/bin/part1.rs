fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let config_input = [("red", 12), ("green", 13), ("blue", 14)];

    let result = input
        .lines()
        .map(|line| {
            let mut result = get_game_number(&line);
            for color in config_input {
                let mut color_name_iter = line.match_indices(color.0);
                let color_validate = color_name_iter.all(|sub_string| {
                    line[sub_string.0 - 3..sub_string.0 - 1]
                        .trim()
                        .to_string()
                        .parse::<u32>()
                        .expect("is a number")
                        .le(&color.1)
                });
                if color_validate == false {
                    result = 0;
                }
                // dbg!(line);
                // dbg!(color.0);
                // dbg!(color_validate);
            }
            result
        })
        .sum::<u32>();

    result.to_string()
}

fn get_game_number(input_line: &str) -> u32 {
    let mut line_iter = input_line
        .split(':')
        .next()
        .expect("string")
        .chars()
        .filter_map(|character| character.to_digit(10));

    let first = line_iter.next().expect("Is a number.");
    let second = match line_iter.next() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}"),
    };
    match line_iter.next() {
        Some(num) => format!("{second}{num}"),
        None => format!("{second}"),
    }
    .parse::<u32>()
    .expect("Is a number.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8".to_string());
    }
}
