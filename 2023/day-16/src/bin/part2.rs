use std::collections::{BTreeMap, BTreeSet, VecDeque};

use itertools::Itertools;
fn main() {
    let input = include_str!(r"./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
enum RayDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct GridLocation {
    x: i32,
    y: i32,
}

impl GridLocation {
    fn shift(&self, values: (i8, i8)) -> GridLocation {
        GridLocation {
            x: self.x + values.0 as i32,
            y: self.y + values.1 as i32,
        }
    }
}

fn part2(input: &str) -> String {
    let num_lines = input.lines().count() as i32;
    let num_cols = input.lines().next().unwrap().chars().count() as i32;
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y_index, line)| {
            line.chars().enumerate().map(move |(x_index, ch)| {
                (
                    GridLocation {
                        x: x_index as i32,
                        y: y_index as i32,
                    },
                    ch as u8,
                )
            })
        })
        .collect::<BTreeMap<GridLocation, u8>>();
    let possible_starts = grid
        .clone()
        .into_keys()
        .filter_map(|location| {
            let selection;
            if location.y == 0 {
                selection = Some((location, RayDirection::Down));
            } else if location.y == num_lines - 1 {
                selection = Some((location, RayDirection::Up));
            } else if location.x == 0 {
                selection = Some((location, RayDirection::Right));
            } else if location.x == num_cols - 1 {
                selection = Some((location, RayDirection::Up));
            } else {
                selection = None;
            }
            selection
        })
        .collect_vec();

    let max_energy = possible_starts
        .iter()
        .map(|option| {
            let mut net_energized: BTreeSet<GridLocation> = BTreeSet::new();
            let mut seen_split_locations: BTreeSet<GridLocation> = BTreeSet::new();
            let mut splits: VecDeque<(GridLocation, RayDirection)> = VecDeque::from([*option]);

            while splits.len() > 0 {
                let (start, direction) = splits.pop_front().unwrap();

                let (mut energized, new_splits) = cast_ray(start, &grid, direction);
                // let energized_copy = energized.clone();
                net_energized.append(&mut energized);
                new_splits.iter().for_each(|new_split| {
                    if let false = seen_split_locations.contains(new_split.0) {
                        seen_split_locations.insert(new_split.0.clone());
                        splits.push_back((*new_split.0, *new_split.1));
                    }
                });
            }
            // println!("Returned {:?} energized", net_energized.len());
            net_energized.len()
        })
        .max();

    max_energy.unwrap().to_string()
}

fn cast_ray(
    start: GridLocation,
    grid: &BTreeMap<GridLocation, u8>,
    initial_direction: RayDirection,
) -> (BTreeSet<GridLocation>, BTreeMap<GridLocation, RayDirection>) {
    let mut energized = BTreeSet::new();
    let mut active_split_locations: BTreeMap<GridLocation, RayDirection> = BTreeMap::new();
    let mut location = start;
    let mut current_heading = initial_direction;
    let mut shift: (i8, i8);
    loop {
        match current_heading {
            RayDirection::Up => shift = (0, -1),
            RayDirection::Down => shift = (0, 1),
            RayDirection::Left => shift = (-1, 0),
            RayDirection::Right => shift = (1, 0),
        }

        // dbg!(location);
        match grid.get(&mut location) {
            // Match '.'
            Some(46) => {
                energized.insert(location);
                location = location.shift(shift);
            }
            // Match '|'
            Some(124) => match current_heading {
                // Don't Split
                RayDirection::Up | RayDirection::Down => {
                    energized.insert(location);
                    location = location.shift(shift);
                    // dbg!(location);
                }
                // Split
                RayDirection::Left | RayDirection::Right => {
                    if energized.contains(&location) {
                        break;
                    }

                    energized.insert(location);
                    active_split_locations.insert(location, RayDirection::Up);
                    current_heading = RayDirection::Down;
                    location = location.shift((0, 1));
                }
            },
            // Match '-'
            Some(45) => match current_heading {
                // Don't Split
                RayDirection::Left | RayDirection::Right => {
                    energized.insert(location);
                    location = location.shift(shift);
                }
                // Split
                RayDirection::Up | RayDirection::Down => {
                    energized.insert(location);
                    active_split_locations.insert(location, RayDirection::Right);
                    current_heading = RayDirection::Left;
                    location = location.shift((-1, 0));
                }
            },
            // Match '\'
            Some(92) => match current_heading {
                RayDirection::Right => {
                    energized.insert(location);
                    current_heading = RayDirection::Down;
                    location = location.shift((0, 1));
                }
                RayDirection::Left => {
                    energized.insert(location);
                    current_heading = RayDirection::Up;
                    location = location.shift((0, -1));
                }
                RayDirection::Down => {
                    energized.insert(location);
                    current_heading = RayDirection::Right;
                    location = location.shift((1, 0));
                }
                RayDirection::Up => {
                    energized.insert(location);
                    current_heading = RayDirection::Left;
                    location = location.shift((-1, 0));
                }
            },
            // Match '/'
            Some(47) => match current_heading {
                RayDirection::Right => {
                    energized.insert(location);
                    current_heading = RayDirection::Up;
                    location = location.shift((0, -1));
                }
                RayDirection::Left => {
                    energized.insert(location);
                    current_heading = RayDirection::Down;
                    location = location.shift((0, 1));
                }
                RayDirection::Down => {
                    energized.insert(location);
                    current_heading = RayDirection::Left;
                    location = location.shift((-1, 0));
                }
                RayDirection::Up => {
                    energized.insert(location);
                    current_heading = RayDirection::Right;
                    location = location.shift((1, 0));
                }
            },
            _ => break,
        }
    }

    (energized, active_split_locations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, "51".to_string());
    }
}
