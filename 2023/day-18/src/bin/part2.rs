use glam::I64Vec2;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::alphanumeric1,
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut num_cubes: i64 = 0;
    let mut grid: Vec<I64Vec2> = Vec::new();
    grid.push(I64Vec2::new(0, 0));

    input.lines().for_each(|line| {
        let (dir, dist) = parse_instruction(line).unwrap().1;
        num_cubes += dist as i64;
        plot_point(&mut grid, dir, dist as usize)
    });
    // "Shoelace" Method for area of polygon with integer vertices
    let sub_area = (grid
        .iter()
        .tuple_windows()
        .fold(0, |acc, (a, b): (&I64Vec2, &I64Vec2)| {
            acc + ((a.x * b.y) - (a.y * b.x))
        })
        / 2)
    .abs();

    // Pick's Theroem
    let interior_cubes = sub_area + 1 - ((num_cubes as i64 - 1) / 2);
    dbg!(sub_area);
    (num_cubes - 1 + interior_cubes).to_string()
}

fn plot_point(grid: &mut Vec<I64Vec2>, direction: Direction, distance: usize) -> () {
    let last_cube = grid.get(grid.len() - 1).unwrap();

    let new_location = match direction {
        Direction::Up => I64Vec2::new(last_cube.x, last_cube.y - distance as i64),
        Direction::Down => I64Vec2::new(last_cube.x, last_cube.y + distance as i64),
        Direction::Left => I64Vec2::new(last_cube.x - distance as i64, last_cube.y),
        Direction::Right => I64Vec2::new(last_cube.x + distance as i64, last_cube.y),
    };

    grid.push(new_location);
}

fn parse_instruction(input: &str) -> IResult<&str, (Direction, u32)> {
    let (wrapped_hex, _) = take_until("(")(input)?;
    let (_, hex) = delimited(tag("(#"), alphanumeric1, tag(")"))(wrapped_hex)?;
    let (dir_code, quant_as_hex) = take(5usize)(hex)?;
    let decimal_number = u32::from_str_radix(quant_as_hex, 16).unwrap();
    let dir_enum = match dir_code {
        "3" => Direction::Up,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "0" => Direction::Right,
        _ => unimplemented!(),
    };
    Ok(("", (dir_enum, decimal_number)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
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
        assert_eq!(result, "952408144115".to_string());
    }
}
