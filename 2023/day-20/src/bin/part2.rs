use itertools::Itertools;
use nom::{
    bytes::{complete::tag, streaming::take_until},
    character::complete::{alpha0, one_of},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use std::{
    collections::{HashMap, VecDeque},
    os::unix::fs::MetadataExt,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PulseType {
    High,
    Low,
}

#[derive(Debug, Clone, Copy)]
struct Pulse {
    signal: PulseType,
    target: &'static str,
    source: &'static str,
}

#[derive(Debug, PartialEq, Eq, Clone)]

enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq, Eq, Clone)]

enum Module {
    FlipFlop(FlipFlopObj),
    Conjunction(ConjunctionObj),
}

impl Module {
    pub fn set_incoming(&mut self, put: Vec<&'static str>) -> () {
        match self {
            Module::FlipFlop(_) => unreachable!(),
            Module::Conjunction(value) => {
                put.iter().for_each(|sending_module| {
                    value.states.insert(&sending_module, PulseType::Low);
                });
                value.in_conn = put;
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::Off
    }
}

impl State {
    fn toggle(&mut self) {
        match &self {
            State::On => *self = State::Off,
            State::Off => *self = State::On,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

struct FlipFlopObj {
    name: &'static str,
    outgoing_conn: Vec<&'static str>,
    state: State,
}

impl FlipFlopObj {
    fn new(name: &'static str, send_to: Vec<&'static str>) -> Self {
        Self {
            name: name,
            outgoing_conn: send_to,
            state: State::default(),
        }
    }

    fn process(&mut self, signal: PulseType) -> Option<Vec<Pulse>> {
        match signal {
            PulseType::High => None,
            PulseType::Low => match self.state {
                State::On => {
                    self.state.toggle();
                    Some(
                        self.outgoing_conn
                            .iter()
                            .map(|out_target| Pulse {
                                signal: PulseType::Low,
                                target: &out_target,
                                source: self.name,
                            })
                            .collect_vec(),
                    )
                }
                State::Off => {
                    self.state.toggle();
                    Some(
                        self.outgoing_conn
                            .iter()
                            .map(|out_target| Pulse {
                                signal: PulseType::High,
                                target: &out_target,
                                source: self.name,
                            })
                            .collect_vec(),
                    )
                }
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]

struct ConjunctionObj {
    name: &'static str,
    outgoing_conn: Vec<&'static str>,
    in_conn: Vec<&'static str>,
    states: HashMap<&'static str, PulseType>,
}

impl ConjunctionObj {
    fn new(name: &'static str, send_to: Vec<&'static str>) -> Self {
        let temp_hash = HashMap::new();

        Self {
            name: name,
            in_conn: vec![],
            outgoing_conn: send_to,
            states: temp_hash,
        }
    }

    fn process(&mut self, signal: PulseType, originator: &'static str) -> Vec<Pulse> {
        self.states
            .entry(originator)
            .and_modify(|current_signal_state| {
                *current_signal_state = signal;
            });
        if let true = self
            .states
            .values()
            .all(|signal_state| signal_state == &PulseType::High)
        {
            self.outgoing_conn
                .iter()
                .map(|out_target| Pulse {
                    signal: PulseType::Low,
                    target: &out_target,
                    source: self.name,
                })
                .collect_vec()
        } else {
            self.outgoing_conn
                .iter()
                .map(|out_target| Pulse {
                    signal: PulseType::High,
                    target: &out_target,
                    source: self.name,
                })
                .collect_vec()
        }
    }
}

fn part1(input: &'static str) -> String {
    let broadcast_targets = parse_broadcast_targets(
        input
            .lines()
            .find(|line| {
                line.split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .contains("broadcaster")
            })
            .unwrap(),
    )
    .unwrap()
    .1;

    // Get all the modules in one vec
    let mut modules = input
        .lines()
        .filter(|line| !line.starts_with("broadcaster"))
        .map(|line| {
            let (kind, name, connections) = parse_module(line).unwrap().1;
            (
                name,
                match kind {
                    '%' => Module::FlipFlop(FlipFlopObj::new(name, connections)),
                    '&' => Module::Conjunction(ConjunctionObj::new(name, connections)),
                    _ => unreachable!(),
                },
            )
        })
        .collect::<HashMap<&str, Module>>();

    // Build a hash map with conj. module name as keys, empty vec as value. We'll fill them with &str for incoming connections.
    let mut conjuctions = modules
        .iter()
        .filter(|(_, module)| match module {
            Module::FlipFlop(_) => false,
            Module::Conjunction(_) => true,
        })
        .map(|(name, _)| (*name, vec![]))
        .collect::<HashMap<&str, Vec<&str>>>();

    //Iterate over all modules and map their outgoing connections to any conjunctions.
    input
        .lines()
        .filter(|line| !line.starts_with("broadcaster"))
        .for_each(|line| {
            let (_, name, connections) = parse_module(line).unwrap().1;
            connections.iter().for_each(|outgoing_connection| {
                conjuctions
                    .entry(&outgoing_connection)
                    .and_modify(|con_in_list| {
                        con_in_list.push(name);
                    });
            })
        });

    conjuctions.iter().for_each(|(con_name, incoming_names)| {
        modules.entry(con_name).and_modify(|value| {
            value.set_incoming(incoming_names.clone());
        });
    });

    let init_pulses = broadcast_targets
        .iter()
        .map(|init_target| Pulse {
            signal: PulseType::Low,
            target: init_target,
            source: "broadcast",
        })
        .collect_vec();

    let mut cycles = 0;
    let mut turned_on = false;

    while !turned_on {
        turned_on = cycle(&mut modules, &init_pulses);
        cycles += 1;
        // dbg!(cycles);
    }

    cycles.to_string()
}

fn cycle(modules: &mut HashMap<&str, Module>, init_pulses: &Vec<Pulse>) -> bool {
    let mut output_on = false;
    let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();

    init_pulses
        .iter()
        .for_each(|init_pulse| pulse_queue.push_back(*init_pulse));

    while let Some(incoming_pulse) = pulse_queue.pop_front() {
        if incoming_pulse.source == "km"
            && incoming_pulse.target == "gq"
            && incoming_pulse.signal == PulseType::High
        {
            println!(
                "{} -> {:?} -> {}",
                incoming_pulse.source, incoming_pulse.signal, incoming_pulse.target
            );

            output_on = true;
        }

        if let Some(active_module) = modules.get_mut(incoming_pulse.target) {
            match active_module {
                Module::FlipFlop(active_ff) => {
                    let get_pulse_result = active_ff.process(incoming_pulse.signal);
                    if let Some(new_pulses) = get_pulse_result {
                        new_pulses
                            .iter()
                            .for_each(|new_pulse| pulse_queue.push_back(*new_pulse))
                    }
                }
                Module::Conjunction(active_conj) => {
                    let new_pulses =
                        active_conj.process(incoming_pulse.signal, incoming_pulse.source);

                    new_pulses
                        .iter()
                        .for_each(|new_pulse| pulse_queue.push_back(*new_pulse))
                }
            }
        }
    }
    output_on
}

fn parse_broadcast_targets(input: &str) -> IResult<&str, Vec<&str>> {
    let list = tag("broadcaster -> ")(input)?;
    separated_list1(tag(", "), alpha0)(list.0)
}

fn parse_module(input: &str) -> IResult<&str, (char, &str, Vec<&str>)> {
    tuple((
        one_of("%&"),
        take_until(" "),
        preceded(tag(" -> "), separated_list1(tag(", "), alpha0)),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(
            "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        );
        assert_eq!(result, "11687500".to_string());
    }

    #[test]
    fn test2() {
        let result = part1(
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        );
        assert_eq!(result, "32000000".to_string());
    }
}
