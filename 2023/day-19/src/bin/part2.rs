use std::{
    cmp::Ordering,
    collections::{BTreeMap, VecDeque},
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

    let mut range_collections: BTreeMap<String, RangeCollection> = BTreeMap::new();
    let mut workflow_queue: VecDeque<(&Workflow, String)> = VecDeque::new();
    workflow_queue.push_back((
        workflows.get(&String::from("in")).unwrap(),
        String::from("st0"),
    ));

    let mut new_rc = RangeCollection::new();
    new_rc.lineage.push(String::from("st"));
    range_collections.insert(String::from("st0"), new_rc);

    while let Some((workflow, range_target)) = workflow_queue.pop_front() {
        let mut base_range_coll = range_collections.remove(&range_target).unwrap();
        workflow
            .filters
            .iter()
            .enumerate()
            .for_each(|(filter_idx, filter)| {
                let key = workflow.name.clone() + &filter_idx.to_string();
                let mut rc = base_range_coll.clone();
                let target = filter.target.get_target_char();
                let split_point = filter.target.get_target_val();
                let new_range = match filter.test_type {
                    Ordering::Less => match target {
                        'x' => rc.x.start..split_point.clone(),
                        'm' => rc.m.start..split_point.clone(),
                        'a' => rc.a.start..split_point.clone(),
                        's' => rc.s.start..split_point.clone(),
                        _ => unreachable!(),
                    },
                    Ordering::Equal => unreachable!(),
                    Ordering::Greater => match target {
                        'x' => split_point.clone() + 1..rc.x.end as u32,
                        'm' => split_point.clone() + 1..rc.m.end as u32,
                        'a' => split_point.clone() + 1..rc.a.end as u32,
                        's' => split_point.clone() + 1..rc.s.end as u32,
                        _ => unreachable!(),
                    },
                };
                let opp_range = match filter.test_type {
                    Ordering::Greater => match target {
                        'x' => rc.x.start..split_point.clone() + 1,
                        'm' => rc.m.start..split_point.clone() + 1,
                        'a' => rc.a.start..split_point.clone() + 1,
                        's' => rc.s.start..split_point.clone() + 1,
                        _ => unreachable!(),
                    },
                    Ordering::Equal => unreachable!(),
                    Ordering::Less => match target {
                        'x' => split_point.clone()..rc.x.end as u32,
                        'm' => split_point.clone()..rc.m.end as u32,
                        'a' => split_point.clone()..rc.a.end as u32,
                        's' => split_point.clone()..rc.s.end as u32,
                        _ => unreachable!(),
                    },
                };

                match target {
                    'x' => {
                        rc.x = new_range;
                        base_range_coll.x = opp_range
                    }
                    'm' => {
                        rc.m = new_range;
                        base_range_coll.m = opp_range
                    }
                    'a' => {
                        rc.a = new_range;
                        base_range_coll.a = opp_range
                    }
                    's' => {
                        rc.s = new_range;
                        base_range_coll.s = opp_range
                    }
                    _ => unreachable!(),
                }
                rc.lineage.push(workflow.name.to_string());

                match &filter.destination {
                    Result::Accept => rc.result = Some(Result::Accept),
                    Result::Reject => rc.result = Some(Result::Reject),
                    Result::Workflow(dest) => {
                        workflow_queue.push_back((workflows.get(dest).unwrap(), key.clone()));
                    }
                }
                range_collections.insert(key, rc);
            });
        base_range_coll.lineage.push(workflow.name.clone());
        // dbg!(&base_range_coll);
        match &workflow.fallthrough {
            Result::Accept => {
                base_range_coll.result = Some(Result::Accept);
                range_collections.insert(workflow.name.clone(), base_range_coll);
            }
            Result::Reject => {
                base_range_coll.result = Some(Result::Reject);
                range_collections.insert(workflow.name.clone(), base_range_coll);
            }
            Result::Workflow(ft) => {
                range_collections.insert(workflow.name.clone(), base_range_coll);
                {
                    workflow_queue.push_back((workflows.get(ft).unwrap(), workflow.name.clone()));
                }
            }
        }
    }

    let accepted = range_collections
        .iter()
        .filter(|(_, collection)| match collection.get_result() {
            Result::Accept => true,
            Result::Reject => false,
            Result::Workflow(_) => false,
        })
        .fold(0, |acc, (_, filtered_range)| acc + filtered_range.length());
    accepted.to_string()
}

fn parse_to_groups(input: &str) -> IResult<&str, &str> {
    terminated(take_until("\n\n"), tag("\n\n"))(input)
}

fn collect_filter(input: &str) -> IResult<&str, Filter> {
    let (dest, test) = terminated(take_until(":"), tag(":"))(input)?;
    let (remaining, parsed_des) = take_until(",")(dest)?;

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

#[allow(dead_code)]
#[derive(Debug)]
struct RangeCollection {
    lineage: Vec<String>,
    result: Option<Result>,
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl RangeCollection {
    fn new() -> Self {
        RangeCollection {
            lineage: Vec::new(),
            result: None,
            x: Range {
                start: 1,
                end: 4001,
            },
            m: Range {
                start: 1,
                end: 4001,
            },
            a: Range {
                start: 1,
                end: 4001,
            },
            s: Range {
                start: 1,
                end: 4001,
            },
        }
    }

    fn clone(&self) -> Self {
        RangeCollection {
            lineage: self.lineage.clone(),
            result: self.result.clone(),
            x: self.x.clone(),
            m: self.m.clone(),
            a: self.a.clone(),
            s: self.s.clone(),
        }
    }
    fn get_result(&self) -> Result {
        self.clone().result.unwrap()
    }

    fn length(&self) -> u64 {
        self.x.len() as u64 * self.m.len() as u64 * self.a.len() as u64 * self.s.len() as u64
    }
}

#[derive(Debug, Clone)]

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

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Filter {
    target: Target,
    test_type: Ordering,
    destination: Result,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
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
