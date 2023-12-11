use itertools::Itertools;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Location {
    x: u32,
    y: u32,
}

fn part1(input: &str) -> String {
    const PREDICT_AT: usize = 100;
    let expansion_test_cases: [usize; 3] = [2, 10, 100];

    let data = expansion_test_cases
        .iter()
        .map(|test_input| {
            let result = get_total_distances(input, *test_input);
            (test_input, result)
        })
        .collect_vec();
    let data_combos = data.iter().combinations(2);
    let rate_of_changes = data_combos
        .map(|combo| ((combo[1].1 - combo[0].1) / (*combo[1].0 as f64 - *combo[0].0 as f64)))
        .collect_vec();
    let predicted = ((PREDICT_AT - expansion_test_cases.last().unwrap()) as f64
        * rate_of_changes.last().unwrap()) as f64
        + data.last().unwrap().1;

    predicted.to_string()
}

fn get_total_distances(og_universe: &str, expansion_factor: usize) -> f64 {
    let expanded_universe = expand(og_universe, expansion_factor);

    let grid = expanded_universe
        .iter()
        .enumerate()
        .flat_map(|(y_index, line)| {
            line.chars().enumerate().map(move |(x_index, character)| {
                let grid_location = Location {
                    x: x_index as u32,
                    y: y_index as u32,
                };
                match character == '#' {
                    true => (grid_location, true),
                    false => (grid_location, false),
                }
            })
        })
        .collect::<BTreeMap<Location, bool>>();

    let galaxies = grid
        .iter()
        .filter_map(|(location, is_galaxy)| is_galaxy.then_some(location))
        .collect_vec();

    let galaxy_combos = galaxies.iter().combinations(2).collect_vec();

    galaxy_combos
        .iter()
        .map(|combo| {
            ((combo[1].x as f64 - combo[0].x as f64).abs()
                + (combo[1].y as f64 - combo[0].y as f64).abs())
            .ceil()
        })
        .sum::<f64>()
}

fn expand(input_universe: &str, expansion_factor: usize) -> Vec<String> {
    let row_length = input_universe.lines().next().unwrap().len();
    let empty_line = ".".repeat(row_length);
    let mut lines = input_universe.lines().map_into::<String>().collect_vec();

    let empty_rows = input_universe
        .lines()
        .enumerate()
        .filter_map(|(line_idx, line)| (!line.contains('#')).then_some(line_idx))
        .collect_vec();

    empty_rows
        .iter()
        .enumerate()
        .for_each(|(loop_idx, empty_row_index)| {
            for _i in 1..expansion_factor {
                // println!("Round {}: Insert #{}", loop_idx, i);
                lines.insert(
                    *empty_row_index + loop_idx * (expansion_factor - 1),
                    empty_line.to_owned(),
                )
            }
        });

    let mut empty_columns: Vec<usize> = Vec::new();

    for i in 0..row_length {
        match lines.iter().all(|line| line.chars().nth(i).unwrap() == '.') {
            true => empty_columns.push(i),
            false => continue,
        }
    }

    empty_columns
        .iter()
        .enumerate()
        .for_each(|(loop_idx, empty_column_index)| {
            for _i in 1..expansion_factor {
                lines.iter_mut().for_each(|line| {
                    line.insert(empty_column_index + loop_idx * (expansion_factor - 1), '.')
                });
            }
        });

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, "8410".to_string());
    }
}
