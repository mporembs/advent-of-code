use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1, space1, u8},
    sequence::{preceded, tuple},
    IResult,
};
struct cube {
    location: IVec2,
    color: String,
}

impl cube {
    fn new(x: i32, y: i32, color: &str) -> Self {
        Self {
            location: IVec2 { x: x, y: y },
            color: color.to_string(),
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let grid: Vec<cube> = Vec::new();
    let instructs = input
        .lines()
        .map(|line| parse_instruction(line))
        .collect_vec();

    dbg!(instructs);

    "0".to_string()
}
fn parse_direction(input: &str) -> IResult<&str, char> {
    anychar(input)
}

fn parse_instruction(input: &str) -> IResult<&str, (char, u8, &'static str)> {
    let (remaining1, direction) = parse_direction(input)?;
    let (remaining2, quant) = preceded(tag(" "), u8)(remaining1)?;
    // let (remaining3, hex) = take;

    Ok((remaining2, (direction, quant, "test")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        );
        assert_eq!(result, "46".to_string());
    }
}
