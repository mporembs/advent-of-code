use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let patterns = input.split("\n\n").collect_vec();
    let total = patterns
        .iter()
        .map(|pattern| {
            let pattern_char_grid = pattern
                .lines()
                .map(|row| row.chars().collect_vec())
                .collect_vec();
            process_pattern(pattern_char_grid).unwrap()

            // 0
        })
        .sum::<usize>();
    total.to_string()
}

fn process_pattern(grid: Vec<Vec<char>>) -> Option<usize> {
    let row = (0..grid.len() - 1).find_map(|i| {
        if test_row(&grid, i) {
            Some(100 * (i + 1))
        } else {
            None
        }
    });
    let col = (0..grid[0].len() - 1).find_map(|i| {
        if test_col(&grid, i) {
            Some(i + 1)
        } else {
            None
        }
    });

    row.or(col)
}

fn test_row(grid: &[Vec<char>], index: usize) -> bool {
    let height = usize::min(index + 1, grid.len() - index - 1);

    (0..height).all(|i| {
        let row_above = &grid[index - i];
        let row_below = &grid[index + i + 1];

        row_above == row_below
    })
}

fn test_col(grid: &[Vec<char>], index: usize) -> bool {
    let width = usize::min(index + 1, grid[0].len() - index - 1);

    (0..width).all(|i| {
        let col_left = grid.iter().map(|row| row[index - i]);
        let col_right = grid.iter().map(|row| row[index + i + 1]);

        itertools::equal(col_left, col_right)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, "405".to_string());
    }
}
