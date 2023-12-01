fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let numbers_words = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    //let result = dbg!(input.split("/n").collect());
    let result = input
        .lines()
        .map(|line| {
            let mut final_string = String::from("");
            let mut start_working_string = String::from(line.trim_start());
            let mut end_working_string = String::from(line.trim_start());

            'start_number_search: loop {
                match start_working_string.chars().nth(0).unwrap().is_numeric() {
                    true => break 'start_number_search,
                    false => {
                        for word in numbers_words {
                            let (num_string, num_char) = word;
                            // dbg!(&start_working_string);

                            if start_working_string.starts_with(&num_string) {
                                start_working_string =
                                    start_working_string.replace(num_string, num_char);
                                break 'start_number_search;
                            }
                        }
                        start_working_string.remove(0);
                    }
                }
            }

            'end_number_search: loop {
                match end_working_string.chars().nth_back(0).unwrap().is_numeric() {
                    true => break 'end_number_search,
                    false => {
                        for word in numbers_words {
                            let (num_string, num_char) = word;
                            // dbg!(&end_working_string);

                            if end_working_string.ends_with(&num_string) {
                                end_working_string =
                                    end_working_string.replace(num_string, num_char);
                                break 'end_number_search;
                            }
                        }
                        end_working_string.remove(end_working_string.len() - 1);
                    }
                }
            }

            // dbg!(&working_string);
            // let sub_string = working_string.trim_matches(char::is_alphabetic);
            final_string.push(start_working_string.char_indices().nth(0).unwrap().1);
            final_string.push(end_working_string.char_indices().nth_back(0).unwrap().1);

            // println!("{}: {}", String::from(line.trim_start()), final_string);
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
        let result = part2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(result, "281".to_string());
    }
}
