//  Solved with DFA. See: https://github.com/ConcurrentCrab/AoC/blob/main/solutions/12-1.go
//
//
use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{newline, space0, u32},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let (_, puzzles) = parse_puzzles(input).unwrap();

    let total_options = puzzles.iter().map(|puzzle| puzzle.solve()).sum::<usize>();

    // dbg!(puzzles);
    total_options.to_string()
}

#[derive(Debug)]
struct Puzzle {
    line: String,
    group_sizes: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    groups_satisfied: usize,
    cont_count: usize,
    expecting_dot: bool,
}

impl State {
    fn new(ci: usize, cc: usize, ed: bool) -> Self {
        State {
            groups_satisfied: ci,
            cont_count: cc,
            expecting_dot: ed,
        }
    }
}

impl Puzzle {
    fn solve(&self) -> usize {
        let mut state_map: HashMap<State, usize> = HashMap::new();
        let mut new_states: HashMap<State, usize> = HashMap::new();
        state_map.insert(State::new(0, 0, false), 1);

        let mut input_chars = self.line.chars();

        while let Some(active_char) = input_chars.next() {
            state_map.iter_mut().for_each(|(state, count)| {
                let State {
                    groups_satisfied: mut ci,
                    cont_count: mut cc,
                    expecting_dot: mut ed,
                } = state;

                let unsatisified = ci < self.group_sizes.len();
                let accumulating = cc.ne(&0);

                match (active_char, unsatisified, ed, accumulating) {
                    ('#' | '?', true, false, _) => {
                        if active_char.eq(&'?') && !accumulating {
                            new_states
                                .entry(State {
                                    groups_satisfied: ci,
                                    cont_count: cc,
                                    expecting_dot: ed,
                                })
                                .and_modify(|current| *current += *count)
                                .or_insert(*count);
                        }
                        cc += 1;
                        if cc == self.group_sizes[ci] as usize {
                            ci += 1;
                            (cc, ed) = (0, true);
                        }
                        new_states
                            .entry(State {
                                groups_satisfied: ci,
                                cont_count: cc,
                                expecting_dot: ed,
                            })
                            .or_insert(*count);
                    }
                    ('.' | '?', _, _, false) => {
                        ed = false;
                        new_states
                            .entry(State {
                                groups_satisfied: ci,
                                cont_count: cc,
                                expecting_dot: ed,
                            })
                            .and_modify(|current| *current += *count)
                            .or_insert(*count);
                    }
                    _ => (),
                }
            });
            (state_map, new_states) = (new_states, state_map);
            new_states.clear();
        }

        let totes_poss = state_map
            .iter()
            .filter_map(|(state, possible)| {
                match state.groups_satisfied == self.group_sizes.len() {
                    true => Some(possible),
                    false => None,
                }
            })
            .sum::<usize>();
        totes_poss
    }
}

fn parse_batches(input: &str) -> IResult<&str, Vec<u32>> {
    let (rem, raw_batches) = separated_list0(tag(","), u32)(input)?;
    let expanded_batches = Vec::from_iter(raw_batches.repeat(5));

    Ok((rem, expanded_batches))
}

fn parse_springs(input: &str) -> IResult<&str, String> {
    let (rem, raw_slice) = take_until(" ")(input)?;
    let mut expanded_slice = raw_slice.repeat(5);
    for i in 1..5 {
        expanded_slice.insert((i * raw_slice.len()) + (i - 1), '?');
    }

    Ok((rem, expanded_slice))
}

fn parse_puzzle(input: &str) -> IResult<&str, Puzzle> {
    let (rem, (springs, batches)) = separated_pair(parse_springs, space0, parse_batches)(input)?;

    Ok((
        rem,
        Puzzle {
            line: springs,
            group_sizes: batches,
        },
    ))
}

fn parse_puzzles(input: &str) -> IResult<&str, Vec<Puzzle>> {
    separated_list0(newline, parse_puzzle)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(result, "525152".to_string());
    }
}
