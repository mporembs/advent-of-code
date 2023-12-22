use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{alphanumeric1, u8},
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Cube {
    location: IVec2,
    #[allow(dead_code)]
    color: String,
}

// impl Cube {
//     fn new(x: i32, y: i32, color: &str) -> Self {
//         Self {
//             location: IVec2 { x: x, y: y },
//             color: color.to_string(),
//         }
//     }
// }

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut grid: Vec<Cube> = Vec::new();
    grid.push(Cube {
        location: IVec2::new(0, 0),
        color: String::from("000000"),
    });

    input.lines().for_each(|line| {
        let (dir, quant, hex) = parse_instruction(line).unwrap().1;
        dig_cube(&mut grid, dir, hex, quant as usize)
    });

    // "Shoelace" Method for area of polygon with integer vertices
    let sub_area =
        (grid
            .iter()
            .tuple_windows()
            .fold(0 as f32, |acc: f32, (a, b): (&Cube, &Cube)| {
                acc as f32
                    + ((a.location.x as f32 * b.location.y as f32)
                        - (a.location.y as f32 * b.location.x as f32))
            })
            / 2 as f32)
            .abs();

    // Pick's Theroem
    let interior_cubes = sub_area + 1f32 - ((grid.len() as f32 - 1f32) / 2f32);

    (grid.len() - 1 + interior_cubes as usize).to_string()
}

fn dig_cube(grid: &mut Vec<Cube>, direction: Direction, color: &str, quant: usize) -> () {
    for _ in 0..quant {
        if grid.is_empty() {
            continue;
        }
        let last_cube = grid.get(grid.len() - 1).unwrap();

        let new_location = match direction {
            Direction::Up => IVec2::new(last_cube.location.x, last_cube.location.y + 1),
            Direction::Down => IVec2::new(last_cube.location.x, last_cube.location.y - 1),
            Direction::Left => IVec2::new(last_cube.location.x - 1, last_cube.location.y),
            Direction::Right => IVec2::new(last_cube.location.x + 1, last_cube.location.y),
        };

        grid.push(Cube {
            location: new_location,
            color: color.to_string(),
        })
    }
}

fn parse_instruction(input: &str) -> IResult<&str, (Direction, u8, &str)> {
    let (remaining1, direction) = take(1usize)(input)?;
    let (remaining2, quant) = preceded(tag(" "), u8)(remaining1)?;
    let (last, hex) = delimited(tag(" (#"), alphanumeric1, tag(")"))(remaining2)?;

    let dir_enum = match direction {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => unimplemented!(),
    };

    Ok((last, (dir_enum, quant, hex)))
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
        assert_eq!(result, "62".to_string());
    }
}
