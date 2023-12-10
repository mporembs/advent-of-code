use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let s_index = input.find('S').unwrap();
    let row_length = input.lines().next().unwrap().len();
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'S' => (5, 5, 5, 5),
                    '|' => (1, 1, 0, 0),
                    '-' => (0, 0, 1, 1),
                    'F' => (0, 1, 1, 0),
                    '7' => (0, 1, 0, 1),
                    'L' => (1, 0, 1, 0),
                    'J' => (1, 0, 0, 1),
                    _ => (0, 0, 0, 0),
                })
                .collect_vec()
        })
        .collect_vec();

    dbg!(input);
    dbg!(grid);
    dbg!(s_index);

    "0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, "8".to_string());
    }
}
