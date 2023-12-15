use itertools::Itertools;

// enum TiltDirection {
//     North,
//     South,
//     East,
//     West,
// }

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let og_grid = input
        .lines()
        .map(|line_str| line_str.to_string())
        .collect_vec();
    let tilted_north = tilt_column_north(og_grid);
    let score = get_weight(tilted_north);
    score.to_string()
}

fn condense_string_left(raw_column: &String) -> String {
    let mut working_column = raw_column.clone();
    let square_indexs = &raw_column
        .match_indices('#')
        .map(|(hash_idx, _)| hash_idx)
        .collect_vec();
    for (count, hash_index) in square_indexs.iter().enumerate() {
        working_column.insert(*hash_index + count, ' ');
    }
    working_column
        .split_ascii_whitespace()
        .map(|sub_string| {
            let quantities = &sub_string.chars().counts();
            let mut condensed_char: Vec<char> = Vec::new();
            if quantities.get(&'#').is_some() {
                (0..quantities[&'#']).for_each(|_| condensed_char.push('#'))
            };
            if quantities.get(&'O').is_some() {
                (0..quantities[&'O']).for_each(|_| condensed_char.push('O'))
            };
            if quantities.get(&'.').is_some() {
                (0..quantities[&'.']).for_each(|_| condensed_char.push('.'))
            };
            condensed_char.iter().collect::<String>()
        })
        .collect::<String>()
}

fn get_weight(grid: Vec<String>) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .filter_map(|(row_idx, row)| match row.chars().counts().get(&'O') {
            Some(count) => Some(count * (row_idx + 1)),
            None => None,
        })
        .sum()
}

fn tilt_column_north(input: Vec<String>) -> Vec<String> {
    let working = rotate_grid(&input)
        .iter()
        .map(|column| condense_string_left(column))
        .collect_vec();
    rotate_grid(&working)
}

fn rotate_grid(input: &Vec<String>) -> Vec<String> {
    let width = input.iter().next().unwrap().len();
    let mut rev_grid: Vec<String> = Vec::new();
    (0..width).for_each(|index| {
        rev_grid.push(
            input
                .iter()
                .map(|line| line.chars().nth(index).unwrap())
                .collect::<Vec<char>>()
                .iter()
                .collect::<String>(),
        );
    });
    rev_grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        assert_eq!(result, "136".to_string());
    }
}
