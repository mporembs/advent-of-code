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
    let universe = expand(input, 10);

    let grid = universe
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

    let distances = galaxy_combos
        .iter()
        .map(|combo| {
            (combo[1].x.abs_diff(combo[0].x) as f64 + combo[1].y.abs_diff(combo[0].y) as f64).ceil()
        })
        .sum::<f64>();

    distances.to_string()
}

fn expand(og_universe: &str, expansion_facter: usize) -> Vec<String> {
    let row_length = og_universe.lines().next().unwrap().len();
    let empty_line = ".".repeat(row_length);
    let mut lines = og_universe.lines().map_into::<String>().collect_vec();

    let empty_rows = og_universe
        .lines()
        .enumerate()
        .filter_map(|(line_idx, line)| (!line.contains('#')).then_some(line_idx))
        .collect_vec();

    empty_rows
        .iter()
        .enumerate()
        .for_each(|(loop_idx, empty_row_index)| {
            for _i in 1..expansion_facter {
                // println!("Round {}: Insert #{}", loop_idx, i);
                lines.insert(
                    *empty_row_index + loop_idx * (expansion_facter - 1),
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
            for _i in 1..expansion_facter {
                lines.iter_mut().for_each(|line| {
                    line.insert(empty_column_index + loop_idx * (expansion_facter - 1), '.')
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
        assert_eq!(result, "1030".to_string());
    }
}
