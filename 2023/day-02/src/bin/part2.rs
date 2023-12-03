fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            let mut line_mins = [0, 0, 0];
            let config_input = [("red", 12), ("green", 13), ("blue", 14)];
            for (idx, color) in config_input.iter().enumerate() {
                let color_name_iter = line.match_indices(&color.0);

                color_name_iter.for_each(|sub_string| {
                    let current_value = line[sub_string.0 - 3..sub_string.0 - 1]
                        .trim()
                        .to_string()
                        .parse::<u32>()
                        .expect("is a number");
                    match current_value.gt(&line_mins[idx]) {
                        true => line_mins[idx] = current_value,
                        false => line_mins[idx] = line_mins[idx],
                    };
                });
            }
            line_mins[0] * line_mins[1] * line_mins[2]
        })
        .sum::<u32>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286".to_string());
    }
}
