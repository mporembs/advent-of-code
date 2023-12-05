use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}
#[derive(Debug)]
enum CharType {
    Symbol(char),
    Empty,
    Number(u32),
}
#[derive(Ord, Eq, PartialOrd, PartialEq, Debug, Clone, Copy, Hash)]
struct GridSpot {
    y_index: i32,
    x_index: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

fn part2(input: &str) -> String {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y_index, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(x_index, character)| {
                    (
                        GridSpot {
                            x_index: x_index as i32,
                            y_index: y_index as i32,
                        },
                        match character {
                            '.' => CharType::Empty,
                            c if c.is_ascii_digit() => {
                                CharType::Number(c.to_digit(10).expect("that it is a number."))
                            }
                            c => CharType::Symbol(c),
                        },
                    )
                })
        })
        .collect::<BTreeMap<GridSpot, CharType>>();

    let mut numbers: Vec<Vec<(GridSpot, u32)>> = vec![];
    for (grid_spot, character) in grid.iter() {
        if let CharType::Number(num) = character {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some((g, _)) => {
                            if g.x_index + 1 == grid_spot.x_index {
                                let last = numbers.iter_mut().last().expect("should exist");
                                last.push((*grid_spot, *num));
                            } else {
                                numbers.push(vec![(*grid_spot, *num)]);
                            }
                        }
                        None => unimplemented!("should't happen"),
                    }
                }
                None => numbers.push(vec![(*grid_spot, *num)]),
            }
            // println!("{:?}", GridSpot)
        }
        // dbg!(&numbers);
    }
    // dbg!(numbers);

    let mut gears: HashMap<GridSpot, Vec<u32>> = HashMap::new();

    for num_list in numbers {
        // (x,y) relative
        let positions: [Coord; 8] = [
            Coord { x: 1, y: 0 },
            Coord { x: 1, y: -1 },
            Coord { x: 0, y: -1 },
            Coord { x: -1, y: -1 },
            Coord { x: -1, y: 0 },
            Coord { x: -1, y: 1 },
            Coord { x: 0, y: 1 },
            Coord { x: 1, y: 1 },
        ];
        let num_positions: Vec<Coord> = num_list
            .iter()
            .map(|(gridspot, _)| Coord {
                x: gridspot.x_index as i32,
                y: gridspot.y_index as i32,
            })
            .collect();
        let pos_to_check: Vec<Coord> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions.iter().map(|outer_pos| Coord {
                    x: outer_pos.x + pos.x_index as i32,
                    y: outer_pos.y + pos.y_index as i32,
                })
            })
            .unique()
            .filter(|num| !num_positions.contains(num))
            .collect::<Vec<Coord>>();

        // dbg!(pos_to_check.len(), pos_to_check);
        pos_to_check.iter().any(|pos| {
            let pos_grid_spot = GridSpot {
                x_index: pos.x,
                y_index: pos.y,
            };
            let value = grid.get(&pos_grid_spot);
            let num: u32 = num_list
                .iter()
                .map(|(_, num)| num.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            if let Some(CharType::Symbol('*')) = value {
                let gear = gears.entry(pos_grid_spot).or_insert(vec![]);
                gear.push(num);
                true
            } else {
                false
            }
        });

        // if is_part_number {
        //     total += num_list
        //         .iter()
        //         .map(|(_, num)| num.to_string())
        //         .collect::<String>()
        //         .parse::<u32>()
        //         .unwrap();
        // }

        // dbg!(is_part_number, n);
    }
    let filtered_gears: u32 = gears
        .iter()
        .filter(|gear_key| gear_key.1.len() == 2)
        .map(|gear| gear.1[0] * gear.1[1])
        .sum();
    filtered_gears.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..",
        );
        assert_eq!(result, "467835".to_string());
    }
}
