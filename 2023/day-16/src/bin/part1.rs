use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn main() {
    let input = include_str!(r"./input1.txt");
    let output = part1(input);
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

fn part1(input: &str) -> String {
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

    let mut net_energized: BTreeSet<GridLocation> = BTreeSet::new();
    let mut seen_split_locations: BTreeSet<GridLocation> = BTreeSet::new();
    let mut splits: VecDeque<(GridLocation, RayDirection)> =
        VecDeque::from([(GridLocation { x: 0, y: 0 }, RayDirection::Right)]);

    while splits.len() > 0 {
        let (start, direction) = splits.pop_front().unwrap();

        let (mut energized, new_splits) = cast_ray(start, &grid, direction);
        net_energized.append(&mut energized);
        new_splits.iter().for_each(|new_split| {
            if let false = seen_split_locations.contains(new_split.0) {
                seen_split_locations.insert(new_split.0.clone());
                splits.push_back((*new_split.0, *new_split.1));
            }
        });
    }
    net_energized.len().to_string()
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
        let result = part1(
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
        assert_eq!(result, "46".to_string());
    }
}
