use std::{
    cmp::Ordering,
    collections::{vec_deque, BTreeMap, VecDeque},
    ops::Range,
};

use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::{alpha1, u32},
    multi::separated_list0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let (_, workflows_group) = parse_to_groups(input).unwrap();

    let workflows = workflows_group
        .lines()
        .map(|line| {
            let wf = parse_workflow(line).unwrap().1;
            let wf_name = String::from(wf.name.clone());
            (wf_name, wf)
        })
        .collect::<BTreeMap<String, Workflow>>();

    let mut range_collections: BTreeMap<&str, Vec<RangeCollection>> = BTreeMap::new();
    let mut workflow_queue: VecDeque<&Workflow> = VecDeque::new();
    workflow_queue.push_back(workflows.get(&String::from("in")).unwrap());

    while let Some(workflow) = workflow_queue.pop_front() {
        dbg!(workflow);
        todo!()
    }

    "0".to_string()
}

fn parse_to_groups(input: &str) -> IResult<&str, &str> {
    terminated(take_until("\n\n"), tag("\n\n"))(input)
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

struct RangeCollection {
    lineage: Vec<String>,
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl RangeCollection {
    fn new() -> Self {
        RangeCollection {
            lineage: Vec::new(),
            x: Range {
                start: 0,
                end: 4000,
            },
            m: Range {
                start: 0,
                end: 4000,
            },
            a: Range {
                start: 0,
                end: 4000,
            },
            s: Range {
                start: 0,
                end: 4000,
            },
        }
    }

    fn clone(&self) -> Self {
        RangeCollection {
            lineage: self.lineage.clone(),
            x: self.x.clone(),
            m: self.x.clone(),
            a: self.x.clone(),
            s: self.x.clone(),
        }
    }
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
        let result = part2(
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
        assert_eq!(result, "167409079868000".to_string());
    }
}
