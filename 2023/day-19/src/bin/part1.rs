use std::{cmp::Ordering, collections::BTreeMap};

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::{alpha1, u32},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let (parts, workflows_group) = parse_to_groups(input).unwrap();

    let mut parts_vec = parts
        .lines()
        .map(|line| parse_part(line).unwrap().1)
        .collect_vec();

    let workflows = workflows_group
        .lines()
        .map(|line| {
            let wf = parse_workflow(line).unwrap().1;
            let wf_name = String::from(wf.name.clone());
            (wf_name, wf)
        })
        .collect::<BTreeMap<String, Workflow>>();

    for part in parts_vec.iter_mut() {
        'outer: loop {
            let bucket = part.get_bucket();
            let workflow = workflows.get(&bucket).unwrap();
            let mut success = false;

            if bucket == "Accepted" || bucket == "Rejected" {
                break 'outer;
            }

            for filter in workflow.filters.iter() {
                let target = &filter.target.get_target_char();
                match filter.test_type {
                    Ordering::Less => {
                        success = part
                            .get_field_value_by_char(target)
                            .unwrap()
                            .lt(filter.target.get_target_val());
                        if success {
                            part.set_bucket(filter.destination.clone());
                            break;
                        }
                    }
                    Ordering::Equal => unreachable!(),
                    Ordering::Greater => {
                        success = part
                            .get_field_value_by_char(target)
                            .unwrap()
                            .gt(filter.target.get_target_val());
                        if success {
                            part.set_bucket(filter.destination.clone());
                            break;
                        }
                    }
                }
            }

            if !success {
                part.set_bucket(workflow.fallthrough.clone());
            }
        }
    }

    let num = parts_vec
        .iter()
        .filter(|part| match part.bucket {
            Result::Accept => true,
            Result::Reject => false,
            Result::Workflow(_) => false,
        })
        .fold(0, |acc, part| acc + (part.x + part.m + part.a + part.s));
    num.to_string()
}

fn parse_to_groups(input: &str) -> IResult<&str, &str> {
    terminated(take_until("\n\n"), tag("\n\n"))(input)
}

fn collect_csv(input: &str) -> IResult<&str, u32> {
    preceded(take(2usize), u32)(input)
}

fn parse_csv(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), collect_csv)(input)
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (remaining, digits) = delimited(tag("{"), parse_csv, tag("}"))(input)?;
    Ok((
        remaining,
        Part {
            x: digits[0],
            m: digits[1],
            a: digits[2],
            s: digits[3],
            bucket: Result::Workflow(String::from("in")),
        },
    ))
}

fn collect_filter(input: &str) -> IResult<&str, Filter> {
    // dbg!(input);
    let (dest, test) = terminated(take_until(":"), tag(":"))(input)?;
    let (remaining, parsed_des) = take_until(",")(dest)?;
    // dbg!(test);
    let (_, (target, test, value)) = tuple((take(1usize), take(1usize), u32))(test)?;

    let fin_target = match target {
        "x" => Target::X(value),
        "m" => Target::M(value),
        "a" => Target::A(value),
        "s" => Target::S(value),
        _ => unreachable!(),
    };

    let fin_test = match test {
        "<" => Ordering::Less,
        ">" => Ordering::Greater,
        _ => unimplemented!(),
    };

    let final_dest = match parsed_des {
        "A" => Result::Accept,
        "R" => Result::Reject,
        wf => Result::Workflow(wf.to_string()),
    };
    // let (name,test,value,dest) = tuple(take(1usize),take(1usize))
    // dbg!(&fin_target);
    // dbg!(&fin_test);
    // dbg!(&final_dest);
    Ok((
        remaining,
        Filter {
            target: fin_target,
            test_type: fin_test,
            destination: final_dest,
        },
    ))
}

fn parse_filters(input: &str) -> IResult<&str, Vec<Filter>> {
    separated_list0(tag(","), collect_filter)(input)
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (remaining, par_name) = take_until("{")(input)?;
    let (remaining, par_filters) = preceded(tag("{"), parse_filters)(remaining)?;
    let (remaining, parsed_ft) = delimited(tag(","), alpha1, tag("}"))(remaining)?;

    let parsed_ft = match parsed_ft {
        "A" => Result::Accept,
        "R" => Result::Reject,
        wf => Result::Workflow(wf.to_string()),
    };

    Ok((
        remaining,
        Workflow {
            name: String::from(par_name),
            filters: par_filters,
            fallthrough: parsed_ft,
        },
    ))
}

#[derive(Debug)]
enum Target {
    X(u32),
    M(u32),
    A(u32),
    S(u32),
}

impl Target {
    fn get_target_char(&self) -> char {
        match self {
            Target::X(_) => 'x',
            Target::M(_) => 'm',
            Target::A(_) => 'a',
            Target::S(_) => 's',
        }
    }

    fn get_target_val(&self) -> &u32 {
        match self {
            Target::X(num) => num,
            Target::M(num) => num,
            Target::A(num) => num,
            Target::S(num) => num,
        }
    }
}

#[derive(Debug, Clone)]
enum Result {
    Accept,
    Reject,
    Workflow(String),
}

#[allow(dead_code)]
#[derive(Debug)]
enum Comparision {
    GreaterThan,
    LessThan,
}

#[derive(Debug)]
struct Part {
    bucket: Result,
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get_field_value_by_char(&self, input: &char) -> Option<&u32> {
        match input {
            'x' => Some(&self.x),
            'm' => Some(&self.m),
            'a' => Some(&self.a),
            's' => Some(&self.s),
            _ => None,
        }
    }
    fn get_bucket(&self) -> String {
        match &self.bucket {
            Result::Accept => String::from("Accepted"),
            Result::Reject => String::from("Rejected"),
            Result::Workflow(wf) => wf.clone(),
        }
    }
    fn set_bucket(&mut self, input: Result) {
        self.bucket = input;
    }
}

#[derive(Debug)]
struct Filter {
    target: Target,
    test_type: Ordering,
    destination: Result,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    filters: Vec<Filter>,
    fallthrough: Result,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(result, "19114".to_string());
    }
}
