use std::collections::BTreeMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Location {
    x: u32,
    y: u32,
}

fn part1(input: &str) -> String {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y_index, line)| {
            line.chars().enumerate().map(move |(x_index, char)| {
                let grid_location = Location {
                    x: x_index as u32,
                    y: y_index as u32,
                };
                match char {
                    'S' => (grid_location, vec!["start"]),
                    '|' => (grid_location, vec!["up", "down"]),
                    '-' => (grid_location, vec!["left", "right"]),
                    'F' => (grid_location, vec!["down", "right"]),
                    '7' => (grid_location, vec!["down", "left"]),
                    'L' => (grid_location, vec!["up", "right"]),
                    'J' => (grid_location, vec!["up", "left"]),
                    _ => (grid_location, vec!["none"]),
                }
            })
        })
        .collect::<BTreeMap<Location, Vec<&str>>>();

    let mut loop_complete = false;
    let mut steps = 1;
    let mut current_source = "up";
    let mut current_postion = Location { x: 0, y: 3 };

    while loop_complete == false {
        // println!("--------------------");
        // println!("Step {:?}", steps);
        // println!("Coming from: {:?}", current_source);
        // println!("Entering {:?}", current_postion);

        let options = grid.get(&current_postion).unwrap();
        // println!("It's options are: {:?}", options);

        let selection = options
            .iter()
            .copied()
            .filter(|option| *option != current_source)
            .collect_vec();
        // println!("Selected {:?}", selection);

        match selection[0] {
            "down" => {
                current_source = "up";
                current_postion.y += 1;
                steps += 1;
            }
            "up" => {
                current_source = "down";
                current_postion.y -= 1;
                steps += 1;
            }
            "left" => {
                current_source = "right";
                current_postion.x -= 1;
                steps += 1;
            }
            "right" => {
                current_source = "left";
                current_postion.x += 1;
                steps += 1;
            }
            _ => {
                loop_complete = true;
            }
        }
        // if steps >= 10 {
        //     loop_complete = true;
        // }
    }
    // dbg!(grid);

    (steps / 2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, "8".to_string());
    }
}
