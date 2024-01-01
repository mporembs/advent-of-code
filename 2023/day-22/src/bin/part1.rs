use std::collections::HashMap;

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
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &'static str) -> String {
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

    let settled_bricks = sorted_bricks
        .iter()
        .fold(vec![], |mut acc: Vec<Brick>, brick| {
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
            let updated_cubes = brick
                .cubes
                .iter()
                .map(|old_cube| IVec3::new(old_cube.x, old_cube.y, old_cube.z - z_diff))
                .collect_vec();
            acc.push(Brick {
                cubes: updated_cubes,
            });
            acc
        });

    let cube_to_brick_id_map = settled_bricks
        .iter()
        .enumerate()
        .flat_map(|(id, set_brick)| set_brick.cubes.iter().map(move |cube| (cube.xyz(), id)))
        .collect::<HashMap<IVec3, usize>>();

    let id_to_brick_map = cube_to_brick_id_map.iter().fold(
        HashMap::<usize, Vec<&IVec3>>::new(),
        |mut map, (cube, id)| {
            map.entry(*id)
                .and_modify(|arr| arr.push(cube))
                .or_insert(vec![cube]);
            map
        },
    );

    let removable = settled_bricks
        .iter()
        .filter(|settled_brick| {
            let max_cubes = settled_brick.cubes.iter().max_set_by_key(|cube| cube.z);
            // let our_id = cube_to_brick_id_map[&max_cubes[0]];
            // let max_z = max_cubes[0].z;

            let supported_bricks = max_cubes
                .iter()
                .filter_map(|cube| {
                    cube_to_brick_id_map.get(&IVec3::new(cube.x, cube.y, cube.z + 1))
                })
                .unique()
                .cloned()
                .collect_vec();

            if supported_bricks.is_empty() {
                return true;
            }
            supported_bricks
                .iter()
                .filter(|supported_brick| {
                    let cubes = id_to_brick_map.get(&supported_brick).unwrap();
                    let min_cubes = cubes.iter().min_set_by_key(|cube| cube.z);

                    min_cubes
                        .iter()
                        .filter_map(|cube| {
                            cube_to_brick_id_map.get(&IVec3::new(cube.x, cube.y, cube.z - 1))
                        })
                        .unique()
                        .count()
                        == 1
                })
                .count()
                == 0
        })
        .count();
    removable.to_string()
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

#[derive(Debug)]
struct Brick {
    cubes: Vec<IVec3>,
}

impl Brick {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(
            "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
        );
        assert_eq!(result, "5".to_string());
    }
}
