use glam::{DVec3, I64Vec3, IVec3};
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
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &'static str) -> String {
    let (_, hailstones) = parse_hailstones(input).unwrap();

    let random_stones = hailstones.into_iter().take(3).collect_vec();
    solve_intersection(random_stones);
    // dbg!(random_stones);

    "0".to_string()
}

fn solve_intersection(input: Vec<HailStone>) {
    use ndarray::prelude::*;
    use ndarray_linalg::Solve;

    let h1 = Hailf64 {
        starting_position: DVec3::new(
            input[0].position.x as f64,
            input[0].position.y as f64,
            input[0].position.z as f64,
        ),
        direction: DVec3::new(
            input[0].velocity.x as f64,
            input[0].velocity.y as f64,
            input[0].velocity.z as f64,
        ),
    };

    // let h1 = Hailf64 {
    //     starting_position: DVec3::new(19., 13., 30.),
    //     direction: DVec3::new(-2., 1., -2.),
    // };
    let v1 = vec![h1.direction.x, h1.direction.y, h1.direction.z];
    let p1 = vec![
        h1.starting_position.x,
        h1.starting_position.y,
        h1.starting_position.z,
    ];

    let h2 = Hailf64 {
        starting_position: DVec3::new(
            input[1].position.x as f64,
            input[1].position.y as f64,
            input[1].position.z as f64,
        ),
        direction: DVec3::new(
            input[1].velocity.x as f64,
            input[1].velocity.y as f64,
            input[1].velocity.z as f64,
        ),
    };
    //    let h2 = Hailf64 {
    //     starting_position: DVec3::new(18., 19., 22.),
    //     direction: DVec3::new(-1., -1., -2.),
    // };

    let v2 = vec![h2.direction.x, h2.direction.y, h2.direction.z];
    let p2 = vec![
        h2.starting_position.x,
        h2.starting_position.y,
        h2.starting_position.z,
    ];
    let h3 = Hailf64 {
        starting_position: DVec3::new(
            input[2].position.x as f64,
            input[2].position.y as f64,
            input[2].position.z as f64,
        ),
        direction: DVec3::new(
            input[2].velocity.x as f64,
            input[2].velocity.y as f64,
            input[2].velocity.z as f64,
        ),
    };
    // let h3 = Hailf64 {
    //     starting_position: DVec3::new(18., 19., 22.),
    //     direction: DVec3::new(-1., -1., -2.),
    // };

    let v3 = vec![h3.direction.x, h3.direction.y, h3.direction.z];
    let p3 = vec![
        h3.starting_position.x,
        h3.starting_position.y,
        h3.starting_position.z,
    ];

    // 19 - 2 * a = 18 - a
    // 13 + b = 19 - b
    //
    // 1 - a
    // -6 + 2b

    let a: Array2<f64> = array![
        [
            -(v1[1] - v2[1]),
            v1[0] - v2[0],
            0.,
            p1[1] - p2[1],
            -(p1[0] - p2[0]),
            0.
        ],
        [
            -(v1[1] - v3[1]),
            v1[0] - v3[0],
            0.,
            p1[1] - p3[1],
            -(p1[0] - p3[0]),
            0.
        ],
        [
            0.,
            -(v1[2] - v2[2]),
            v1[1] - v2[1],
            0.,
            p1[2] - p2[2],
            -(p1[1] - p2[1])
        ],
        [
            0.,
            -(v1[2] - v3[2]),
            v1[1] - v3[1],
            0.,
            p1[2] - p3[2],
            -(p1[1] - p3[1])
        ],
        [
            -(v1[2] - v2[2]),
            0.,
            v1[0] - v2[0],
            p1[2] - p2[2],
            0.,
            -(p1[0] - p2[0])
        ],
        [
            -(v1[2] - v3[2]),
            0.,
            v1[0] - v3[0],
            p1[2] - p3[2],
            0.,
            -(p1[0] - p3[0])
        ]
    ];
    let b: Array1<f64> = array![
        (p1[1] * v1[0] - p2[1] * v2[0]) - (p1[0] * v1[1] - p2[0] * v2[1]),
        (p1[1] * v1[0] - p3[1] * v3[0]) - (p1[0] * v1[1] - p3[0] * v3[1]),
        (p1[2] * v1[1] - p2[2] * v2[1]) - (p1[1] * v1[2] - p2[1] * v2[2]),
        (p1[2] * v1[1] - p3[2] * v3[1]) - (p1[1] * v1[2] - p3[1] * v3[2]),
        (p1[2] * v1[0] - p2[2] * v2[0]) - (p1[0] * v1[2] - p2[0] * v2[2]),
        (p1[2] * v1[0] - p3[2] * v3[0]) - (p1[0] * v1[2] - p3[0] * v3[2])
    ];
    let coefficients = a.solve_into(b).unwrap();
    dbg!(coefficients);

    // assert_eq!(hail_one.at(xx), hail_two.at(yy));
    // hail_one.at(xx)
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

#[derive(Debug)]
struct Hailf64 {
    starting_position: DVec3,
    direction: DVec3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part2(
            "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3",
        );
        assert_eq!(result, "47".to_string());
    }
}
