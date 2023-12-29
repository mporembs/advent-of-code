use std::collections::{HashMap, HashSet};

use glam::IVec2;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &'static str) -> String {
    let grid = to_grid(input);
    let start = grid
        .iter()
        .filter(|(_, plot)| match plot {
            PlotType::Garden => false,
            PlotType::Rock => false,
            PlotType::Start => true,
        })
        .take(1)
        .next()
        .unwrap();
    let mut reachable: HashSet<IVec2> = HashSet::from([start.0.to_owned()]);

    for _ in 0..64 {
        let mut new_reachable: HashSet<IVec2> = HashSet::new();
        reachable.drain().for_each(|position| {
            let possible: Vec<IVec2> = Vec::from([
                IVec2 {
                    x: position.x.clone(),
                    y: position.y.clone() + 1i32,
                },
                IVec2 {
                    x: position.x.clone(),
                    y: position.y.clone() - 1i32,
                },
                IVec2 {
                    x: position.x.clone() - 1i32,
                    y: position.y.clone(),
                },
                IVec2 {
                    x: position.x.clone() + 1i32,
                    y: position.y.clone(),
                },
            ]);
            possible
                .iter()
                .for_each(|new_poss| match grid.get(new_poss).unwrap() {
                    PlotType::Garden => {
                        new_reachable.insert(*new_poss);
                    }
                    PlotType::Rock => (),
                    PlotType::Start => {
                        new_reachable.insert(*new_poss);
                    }
                });
        });
        reachable = new_reachable;
    }
    // dbg!(grid);
    reachable.len().to_string()
}

fn to_grid(input: &str) -> HashMap<IVec2, PlotType> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y_index, line)| {
            line.chars().enumerate().map(move |(x_index, ch)| {
                (
                    IVec2 {
                        x: x_index as i32,
                        y: y_index as i32,
                    },
                    match ch {
                        '.' => PlotType::Garden,
                        '#' => PlotType::Rock,
                        'S' => PlotType::Start,
                        _ => unreachable!(),
                    },
                )
            })
        })
        .collect::<HashMap<IVec2, PlotType>>()
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum PlotType {
    Garden,
    Rock,
    Start,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
        );
        assert_eq!(result, "16".to_string());
    }
}
