use glam::IVec2;
use std::{collections::HashSet, iter::successors};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &'static str) -> String {
    let skip_num = 6;
    let (start, grid) = to_grid(input);
    // let starting_map: HashSet<IVec2> = HashSet::from([start]);

    let last_set_len = get_reachable_at(skip_num, &grid, &start);

    last_set_len.to_string()
}

fn get_reachable_at(steps: usize, grid: &HashSet<IVec2>, start: &IVec2) -> usize {
    let starting_map: HashSet<IVec2> = HashSet::from([start.to_owned()]);

    successors(Some(starting_map), |reachable_positions| {
        let mut new_reachable: HashSet<IVec2> = HashSet::new();

        for pos in reachable_positions.into_iter() {
            [IVec2::X, IVec2::NEG_X, IVec2::NEG_Y, IVec2::Y]
                .into_iter()
                .filter_map(|step| {
                    let new_pos = *pos + step;
                    grid.contains(&new_pos).then_some(new_pos)
                })
                .for_each(|new| {
                    new_reachable.insert(new);
                })
        }
        Some(new_reachable)
    })
    .skip(steps)
    .next()
    .unwrap()
    .len()
}

fn to_grid(input: &str) -> (IVec2, HashSet<IVec2>) {
    let start = IVec2 {
        x: (input.lines().next().unwrap().chars().count() / 2) as i32,
        y: (input.lines().count() / 2) as i32,
    };
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y_index, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x_index, ch)| match ch {
                    '.' | 'S' => Some(IVec2::new(x_index as i32, y_index as i32)),
                    _ => None,
                })
        })
        .collect::<HashSet<IVec2>>();
    // dbg!(&start);
    (start, grid)
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
