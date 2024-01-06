use glam::{I64Vec3, IVec3};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, i64, newline},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &'static str) -> String {
    let x_bounds = 200000000000000.0..=400000000000000.0;
    let y_bounds = 200000000000000.0..=400000000000000.0;

    let (_, hailstones) = parse_hailstones(input).unwrap();

    let count = hailstones
        .iter()
        .tuple_combinations::<(_, _)>()
        .filter(|(stone_a, stone_b)| {
            let slope_a = stone_a.velocity.y as f64 / stone_a.velocity.x as f64;
            let slope_b = stone_b.velocity.y as f64 / stone_b.velocity.x as f64;

            if let true = slope_a.eq(&slope_b) {
                return false;
            }

            let a_y_inter = stone_a.position.y as f64 - (slope_a * stone_a.position.x as f64);
            let b_y_inter = stone_b.position.y as f64 - (slope_b * stone_b.position.x as f64);

            let cross_x = (b_y_inter - a_y_inter) / (slope_a - slope_b);
            if !x_bounds.contains(&cross_x) {
                return false;
            }
            match stone_a.velocity.x.is_positive() {
                true => {
                    if (stone_a.position.x as f64) > cross_x {
                        return false;
                    }
                }
                false => {
                    if (stone_a.position.x as f64) < cross_x {
                        return false;
                    }
                }
            }
            match stone_b.velocity.x.is_positive() {
                true => {
                    if (stone_b.position.x as f64) > cross_x {
                        return false;
                    }
                }
                false => {
                    if (stone_b.position.x as f64) < cross_x {
                        return false;
                    }
                }
            }

            let cross_y = slope_a * cross_x + a_y_inter;
            if !y_bounds.contains(&cross_y) {
                return false;
            }
            true
        })
        .count();
    count.to_string()
}

fn get_ivec3(input: &str) -> IResult<&str, IVec3> {
    let (rem, coords) = separated_list0(tag(", "), i32)(input)?;
    Ok((
        rem,
        IVec3 {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        },
    ))
}
fn get_i64vec3(input: &str) -> IResult<&str, I64Vec3> {
    let (rem, coords) = separated_list0(tag(", "), i64)(input)?;
    Ok((
        rem,
        I64Vec3 {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        },
    ))
}

fn get_hailstone(input: &str) -> IResult<&str, HailStone> {
    let (rem, ivecs) = separated_pair(get_i64vec3, tag(" @ "), get_ivec3)(input)?;
    let stone = HailStone {
        position: ivecs.0,
        velocity: ivecs.1,
    };
    Ok((rem, stone))
}

fn parse_hailstones(input: &str) -> IResult<&str, Vec<HailStone>> {
    separated_list0(newline, get_hailstone)(input)
}

#[derive(Debug)]
struct HailStone {
    position: I64Vec3,
    velocity: IVec3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(
            "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3",
        );
        assert_eq!(result, "2".to_string());
    }
}
