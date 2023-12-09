use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let sequences = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number| number.parse::<i128>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let new_elements = sequences
        .iter()
        .map(|sequence| sequence.first().unwrap() - next_num(sequence.to_vec()))
        .sum::<i128>();
    new_elements.to_string()
}

fn next_num(og_vec: Vec<i128>) -> i128 {
    match og_vec.iter().all_equal_value() {
        Ok(0) => 0,
        Ok(_) => {
            let mut new_vec = Vec::new();
            for i in 0..og_vec.len() - 1 {
                new_vec.push(og_vec[i + 1] - og_vec[i]);
            }
            // dbg!(&new_vec);
            let first = new_vec.first().unwrap().to_owned();
            first - next_num(new_vec)
        }
        Err(Some((_, _))) => {
            let mut new_vec = Vec::new();
            for i in 0..og_vec.len() - 1 {
                new_vec.push(og_vec[i + 1] - og_vec[i]);
            }
            // dbg!(&new_vec);

            let first = new_vec.first().unwrap().to_owned();
            first - next_num(new_vec)
        }
        Err(None) => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, "2".to_string());
    }
}
