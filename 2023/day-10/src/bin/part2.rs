use std::collections::BTreeMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
struct GridPosition {
    x: u32,
    y: u32,
}

fn part2(input: &str) -> String {
    let mut grid = input
        .lines()
        .enumerate()
        .flat_map(|(y_index, line)| {
            line.chars().enumerate().map(move |(x_index, char)| {
                let grid_location = GridPosition {
                    x: x_index as u32,
                    y: y_index as u32,
                };
                match char {
                    'S' => (grid_location, (vec!["start"], true)),
                    '|' => (grid_location, (vec!["up", "down"], false)),
                    '-' => (grid_location, (vec!["left", "right"], false)),
                    'F' => (grid_location, (vec!["down", "right"], false)),
                    '7' => (grid_location, (vec!["down", "left"], false)),
                    'L' => (grid_location, (vec!["up", "right"], false)),
                    'J' => (grid_location, (vec!["up", "left"], false)),
                    '.' => (grid_location, (vec!["ground"], false)),
                    _ => (grid_location, (vec!["none"], false)),
                }
            })
        })
        .collect::<BTreeMap<GridPosition, (Vec<&str>, bool)>>();
    // let mut grid_path_record = grid.iter().map(|record| record.clone()).collect::<BTreeMap<Location, (Vec<&str>, bool)>>();

    let mut tiles = 0f32;
    let mut vertex_sequence = Vec::new();
    let mut loop_complete = false;
    let mut current_source = "up";
    let origin = GridPosition { x: 28, y: 32 };
    let mut current_postion = GridPosition { x: 28, y: 32 };
    while loop_complete == false {
        // println!("--------------------");
        // println!("Step {:?}", steps);
        // println!("Coming from: {:?}", current_source);
        // println!("Entering {:?}", current_postion);
        grid.entry(current_postion).and_modify(|v| v.1 = true);
        vertex_sequence.push(current_postion.clone());
        let options = grid.get(&current_postion).unwrap();
        // println!("It's options are: {:?}", options);

        let selection = options
            .0
            .iter()
            .copied()
            .filter(|option| *option != current_source)
            .collect_vec();
        // println!("Selected {:?}", selection);

        match selection[0] {
            "down" => {
                current_source = "up";
                current_postion.y += 1;
            }
            "up" => {
                current_source = "down";
                current_postion.y -= 1;
            }
            "left" => {
                current_source = "right";
                current_postion.x -= 1;
            }
            "right" => {
                current_source = "left";
                current_postion.x += 1;
            }
            _ => {
                vertex_sequence.push(origin);
                loop_complete = true;
            }
        }
        // if steps >= 10 {
        //     loop_complete = true;
        // }
    }

    // "Shoelace" Method for area of polygon with integer vertices
    let sub_area = vertex_sequence.iter().tuple_windows().fold(
        0 as f32,
        |acc: f32, (a, b): (&GridPosition, &GridPosition)| {
            // println!(
            //     "{}x{} - {}x{}",
            //     a.x as u32, b.y as u32, a.y as u32, b.x as u32
            // );
            acc as f32 + ((a.x as f32 * b.y as f32) - (a.y as f32 * b.x as f32))
        },
    ) / 2 as f32;
    // println!(
    //     "Vertices: {} , Area: {}",
    //     vertex_sequence.len() - 1,
    //     sub_area
    // );

    // Pick's Theroem
    tiles = sub_area + 1f32 - ((vertex_sequence.len() as f32 - 1f32) / 2f32);
    // dbg!(sub_area / 2);
    tiles.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        );
        assert_eq!(result, "10".to_string());
    }
}
