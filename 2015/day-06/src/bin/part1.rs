use std::{collections::HashMap, ops::Range};

use glam::IVec2;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{i32, newline},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut light_status: HashMap<IVec2, bool> = HashMap::new();

    (0..1_000).for_each(|row| {
        (0..1_000).for_each(|col| {
            light_status.insert(IVec2::new(col, row), false);
        })
    });

    let (_, instuctions) = parse_instructs(input).unwrap();
    // dbg!(&instuctions);
    instuctions.iter().for_each(|instuction| {
        instuction.row_range.clone().for_each(|row_idx| {
            instuction.col_range.clone().for_each(|col_idx| {
                match instuction.command {
                    LightCommand::On => light_status
                        .entry(IVec2::new(col_idx as i32, row_idx as i32))
                        .and_modify(|status| *status = true),
                    LightCommand::Off => light_status
                        .entry(IVec2::new(col_idx as i32, row_idx as i32))
                        .and_modify(|status| *status = false),
                    LightCommand::Toggle => {
                        // dbg!("test");
                        // dbg!(IVec2::new(col_idx as i32, row_idx as i32));
                        light_status
                            .entry(IVec2::new(col_idx as i32, row_idx as i32))
                            .and_modify(|status| match status {
                                true => *status = false,
                                false => *status = true,
                            })
                    }
                };
            })
        });
    });

    let on = light_status.values().filter(|&&status| status).count();
    on.to_string()
}

fn parse_range(input: &str) -> IResult<&str, IVec2> {
    let (input, coords) = separated_pair(i32, tag(","), i32)(input)?;
    Ok((input, IVec2::new(coords.0, coords.1)))
}

fn parse_command(input: &str) -> IResult<&str, &str> {
    take_till(|c: char| c.is_digit(10))(input)
}

fn parse_single_inst(input: &str) -> IResult<&str, Instruction> {
    let (input, command_str) = parse_command(input)?;
    let (input, ranges) = separated_pair(parse_range, tag(" through "), parse_range)(input)?;
    Ok((
        input,
        Instruction::new(
            ranges.0.x as u16,
            ranges.1.x as u16,
            ranges.0.y as u16,
            ranges.1.y as u16,
            match command_str {
                "turn on " => LightCommand::On,
                "turn off " => LightCommand::Off,
                "toggle " => LightCommand::Toggle,
                _ => unreachable!(),
            },
        ),
    ))
}

fn parse_instructs(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(newline, parse_single_inst)(input)
}

#[derive(Debug)]
struct Instruction {
    row_range: Range<u16>,
    col_range: Range<u16>,
    command: LightCommand,
}

#[derive(Debug)]
enum LightCommand {
    On,
    Off,
    Toggle,
}

impl Instruction {
    fn new(row_start: u16, row_end: u16, col_start: u16, col_end: u16, todo: LightCommand) -> Self {
        Instruction {
            row_range: Range {
                start: row_start,
                end: row_end + 1,
            },
            col_range: Range {
                start: col_start,
                end: col_end + 1,
            },
            command: todo,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500",
        );
        assert_eq!(result, "998996".to_string());
    }
}
