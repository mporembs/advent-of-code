use std::collections::HashSet;

use glam::IVec2;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let origin = IVec2::ZERO;
    let mut visited = HashSet::from([origin]);

    input
        .chars()
        .tuples::<(_, _)>()
        .fold((origin, origin), |curr, ch| {
            let mut new: (IVec2, IVec2) = (IVec2::ZERO, IVec2::ZERO);
            match ch.0 {
                '^' => new.0 = IVec2::Y + curr.0,
                '<' => new.0 = IVec2::NEG_X + curr.0,
                '>' => new.0 = IVec2::X + curr.0,
                'v' => new.0 = IVec2::NEG_Y + curr.0,
                _ => unreachable!(),
            };
            visited.insert(new.0);
            match ch.1 {
                '^' => new.1 = IVec2::Y + curr.1,
                '<' => new.1 = IVec2::NEG_X + curr.1,
                '>' => new.1 = IVec2::X + curr.1,
                'v' => new.1 = IVec2::NEG_Y + curr.1,
                _ => unreachable!(),
            };
            visited.insert(new.1);
            new
        });

    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("^v^v^v^v^v");
        assert_eq!(result, "11".to_string());
    }
}
