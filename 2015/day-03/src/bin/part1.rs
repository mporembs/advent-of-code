use std::collections::HashSet;

use glam::IVec2;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let origin = IVec2::ZERO;
    let mut visited = HashSet::from([origin]);

    input.chars().fold(origin, |curr, ch| {
        let new: IVec2;
        match ch {
            '^' => new = IVec2::Y + curr,
            '<' => new = IVec2::NEG_X + curr,
            '>' => new = IVec2::X + curr,
            'v' => new = IVec2::NEG_Y + curr,
            _ => unreachable!(),
        };
        visited.insert(new);
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
        assert_eq!(result, "2".to_string());
    }
}
