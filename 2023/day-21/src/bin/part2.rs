use glam::IVec2;
use itertools::Itertools;
use std::{collections::HashSet, iter::successors};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &'static str) -> String {
    let desired_steps = 26501365;

    let grid_bounds = IVec2 {
        x: (input.lines().next().unwrap().chars().count()) as i32,
        y: input.lines().count() as i32,
    };

    let single_grid_width = grid_bounds.x as usize;
    let fill_remainder = desired_steps % single_grid_width;
    let final_n: usize = (desired_steps - fill_remainder) / single_grid_width;
    let n_values = 1..=4;
    let (start, grid) = to_grid(input);

    let interval_data = n_values
        .clone()
        .map(|num| {
            get_reachable_at(
                num * single_grid_width + fill_remainder,
                &grid,
                &start,
                grid_bounds,
            )
        })
        .collect_vec();

    let a = get_uniform_diffs(&interval_data) / 2;
    let an2 = n_values.clone().map(|n| a * n.pow(2)).collect_vec();
    let b_rem = &interval_data
        .iter()
        .enumerate()
        .map(|(idx, reachable)| reachable - an2[idx])
        .collect_vec();
    let b = get_uniform_diffs(&b_rem);
    let bn = n_values.map(|n| b * n).collect_vec();
    let c = b_rem
        .iter()
        .enumerate()
        .map(|(idx, b_rem)| b_rem - bn[idx])
        .unique()
        .next()
        .unwrap();

    let final_reachable = do_quad(final_n, a, b, c);
    // assert_eq!(final_reachable, 627_960_775_905_777);
    final_reachable.to_string()
}

fn get_uniform_diffs(input: &Vec<usize>) -> usize {
    let diffs = input
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    match diffs.iter().all_equal_value() {
        Ok(common_diff) => *common_diff,
        Err(_) => get_uniform_diffs(&diffs),
    }
}

fn get_reachable_at(steps: usize, grid: &HashSet<IVec2>, start: &IVec2, bounds: IVec2) -> usize {
    let starting_map: HashSet<IVec2> = HashSet::from([start.to_owned()]);
    successors(Some(starting_map), |reachable_positions| {
        let mut new_reachable: HashSet<IVec2> = HashSet::new();

        for pos in reachable_positions.into_iter() {
            [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
                .into_iter()
                .filter_map(|step| {
                    let new_pos = *pos + step;
                    // dbg!(new_pos, new_pos.rem_euclid(grid_max));
                    grid.contains(&new_pos.rem_euclid(bounds))
                        .then_some(new_pos)
                })
                .for_each(|new| {
                    new_reachable.insert(new);
                })
        }
        Some(new_reachable)
    })
    // .inspect(|set| {
    //     dbg!(set.len());
    // })
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
    // dbg!(start);
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

fn do_quad(n: usize, a: usize, b: usize, c: usize) -> usize {
    a * n.pow(2) + b * n + c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part2(
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
        assert_eq!(result, "6536".to_string());
    }
}
