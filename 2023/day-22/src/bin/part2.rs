use glam::{IVec3, Vec3Swizzles};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &'static str) -> String {
    let bricks = parse_bricks(input).unwrap().1;
    let sorted_bricks = bricks
        .iter()
        .sorted_by(|a, b| {
            a.cubes
                .iter()
                .map(|cube| cube.z)
                .min()
                .unwrap()
                .cmp(&b.cubes.iter().map(|cube| cube.z).min().unwrap())
        })
        .collect_vec();

    let (settled_bricks, _) = fall(sorted_bricks);

    let total_fallen = settled_bricks
        .iter()
        .map(|brick| {
            let new_bricks = settled_bricks.iter().filter(|b| b != &brick).collect_vec();
            let (_, fallen_num) = fall(new_bricks);
            fallen_num
        })
        .sum::<i32>();

    total_fallen.to_string()
}

fn fall(sorted_bricks: Vec<&Brick>) -> (Vec<Brick>, i32) {
    let (fallen_bricks, fall_count) = sorted_bricks.iter().fold(
        (vec![], 0),
        |(mut acc, mut fall_count): (Vec<Brick>, i32), brick| {
            let min_cubes = brick.cubes.iter().min_set_by_key(|cube| cube.z);
            let min_cubes_xy = min_cubes.iter().map(|min_cube| min_cube.xy()).collect_vec();
            let max_acc_z = acc
                .iter()
                .flat_map(|brick| brick.cubes.iter())
                .filter_map(|cube| min_cubes_xy.contains(&cube.xy()).then_some(cube.z))
                .max()
                .unwrap_or(0);
            let insert_z = max_acc_z + 1;
            let brick_z = min_cubes.iter().next().unwrap().z;
            let z_diff = brick_z - insert_z;
            if z_diff >= 1 {
                fall_count += 1;
            }
            let updated_cubes = brick
                .cubes
                .iter()
                .map(|old_cube| IVec3::new(old_cube.x, old_cube.y, old_cube.z - z_diff))
                .collect_vec();
            acc.push(Brick {
                cubes: updated_cubes,
            });
            (acc, fall_count)
        },
    );
    (fallen_bricks, fall_count)
}

fn parse_iv3(input: &str) -> IResult<&str, IVec3> {
    let (rem, coords) = separated_list1(tag(","), i32)(input)?;
    Ok((
        rem,
        IVec3 {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        },
    ))
}

fn get_brick(input: &str) -> IResult<&str, Brick> {
    let (rem, (start, end)) = separated_pair(parse_iv3, char('~'), parse_iv3)(input)?;
    let cubes = [start.x..=end.x, start.y..=end.y, start.z..=end.z]
        .into_iter()
        .multi_cartesian_product()
        .map(|cube| IVec3::new(cube[0], cube[1], cube[2]))
        .collect_vec();

    let brick = Brick { cubes: cubes };
    Ok((rem, brick))
}

fn parse_bricks(input: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(tag("\n"), get_brick)(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Brick {
    cubes: Vec<IVec3>,
}

impl Brick {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part2(
            "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
        );
        assert_eq!(result, "7".to_string());
    }
}
