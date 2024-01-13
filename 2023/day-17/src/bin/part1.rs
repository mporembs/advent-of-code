use glam::IVec2;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;

// Huge struggle, heavy borrowing from:
// https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/17.rs
//
////
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: IVec2,
    direction: EdgeDirection,
}

impl State {
    fn new(r: i32, c: i32, d: EdgeDirection) -> Self {
        State {
            position: IVec2::new(c, r),
            direction: d,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]

enum EdgeDirection {
    Start,
    Right,
    Down,
    Left,
    Up,
}

impl EdgeDirection {
    fn reverse(&self) -> EdgeDirection {
        match &self {
            EdgeDirection::Right => EdgeDirection::Left,
            EdgeDirection::Down => EdgeDirection::Up,
            EdgeDirection::Left => EdgeDirection::Right,
            EdgeDirection::Up => EdgeDirection::Down,
            EdgeDirection::Start => EdgeDirection::Start,
        }
    }
    fn coords(&self) -> IVec2 {
        match &self {
            EdgeDirection::Right => IVec2::X,
            EdgeDirection::Down => IVec2::Y,
            EdgeDirection::Left => IVec2::NEG_X,
            EdgeDirection::Up => IVec2::NEG_Y,
            EdgeDirection::Start => IVec2::ZERO,
        }
    }
}

fn main() {
    let input = include_str!(r"./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let grid = input.lines().map(str::as_bytes).collect_vec();
    let min_goal_score = dijkstra(&grid, 4, 10).unwrap_or(0);

    min_goal_score.to_string()
}

fn dijkstra(grid: &[&[u8]], min_step: u8, max_step: u8) -> Option<i64> {
    let start = State::new(0, 0, EdgeDirection::Start);
    let goal = IVec2::new(grid[0].len() as i32 - 1, grid.len() as i32 - 1);

    let mut scores: HashMap<State, i64> = HashMap::new();
    let mut queue: BinaryHeap<MinScored<i64, State>> = BinaryHeap::from_iter([MinScored(0, start)]);

    while let Some(MinScored(cost, state)) = queue.pop() {
        if state.position == goal {
            return Some(cost);
        }

        if scores
            .get(&state)
            .is_some_and(|&cached_score| -cost > cached_score)
        {
            continue;
        }

        for step_dir in [
            EdgeDirection::Down,
            EdgeDirection::Up,
            EdgeDirection::Left,
            EdgeDirection::Right,
        ]
        .iter()
        .filter(|&&dir| dir != state.direction && dir != state.direction.reverse())
        {
            let mut next_cost = cost;
            // dbg!(step_dir);
            for dist in 1..=max_step {
                let new_row = (state.position.y + step_dir.coords().y * dist as i32) as usize;
                let new_col = (state.position.x + step_dir.coords().x * dist as i32) as usize;
                if new_row >= grid.len() || new_col >= grid[0].len() {
                    break;
                }
                next_cost += (grid[new_row][new_col] - b'0') as i64;
                // println!("x:{new_col}, y:{new_row}, cost:{next_cost}");

                if dist < min_step {
                    continue;
                }

                let new_state = State::new(new_row as i32, new_col as i32, step_dir.clone());

                if next_cost < *scores.get(&new_state).unwrap_or(&i64::MAX) {
                    scores.insert(new_state, next_cost);
                    queue.push(MinScored(next_cost, new_state));
                }
            }
        }
    }

    None
}

//  Makes the default BinaryHeap operate as minHeap, with "Reverse" stuff. Copied from Petgraph
//  A* algo: https://github.com/petgraph/petgraph/blob/master/src/scored.rs
//
/////
#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>(pub K, pub T);

impl<K: PartialOrd, T> PartialEq for MinScored<K, T> {
    #[inline]
    fn eq(&self, other: &MinScored<K, T>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K: PartialOrd, T> Eq for MinScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MinScored<K, T> {
    #[inline]
    fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: PartialOrd, T> Ord for MinScored<K, T> {
    #[inline]
    fn cmp(&self, other: &MinScored<K, T>) -> Ordering {
        let a = &self.0;
        let b = &other.0;
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else if a.ne(a) && b.ne(b) {
            // these are the NaN cases
            Ordering::Equal
        } else if a.ne(a) {
            // Order NaN less, so that it is last in the MinScore order
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        );
        assert_eq!(result, "102".to_string());
    }
}
