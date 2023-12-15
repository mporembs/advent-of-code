use itertools::Itertools;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Copy, Clone, Ord, Eq, PartialEq, PartialOrd)]
struct Lense<'a> {
    label: &'a str,
    focal_length: Option<u8>,
}

fn part2(input: &str) -> String {
    let mut boxes: BTreeMap<u8, Vec<Lense>> = BTreeMap::new();
    for i in 0..255 {
        boxes.insert(i, vec![]);
    }

    let steps = input.split(',').collect_vec();

    steps.iter().for_each(|step| match step.contains('-') {
        true => {
            let lense = Lense {
                label: step.split('-').next().unwrap(),
                focal_length: None,
            };
            boxes.entry(hash(lense.label)).and_modify(|lens_set| {
                if let Some(existing_index) = lens_set
                    .clone()
                    .iter()
                    .position(|installed_lense| installed_lense.label == lense.label)
                {
                    lens_set.remove(existing_index);
                }
            });
        }
        false => {
            let mut step_iter = step.split('=');
            let lense = Lense {
                label: step_iter.next().unwrap(),
                focal_length: Some(step_iter.next().unwrap().parse::<u8>().unwrap()),
            };
            boxes.entry(hash(lense.label)).and_modify(|lens_set| {
                match lens_set
                    .clone()
                    .iter()
                    .position(|installed_lense| installed_lense.label == lense.label)
                {
                    Some(existing_index) => {
                        lens_set.remove(existing_index);
                        lens_set.insert(existing_index, lense)
                    }
                    None => lens_set.push(lense),
                }
            });
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(box_idx, lense_box)| {
            lense_box
                .1
                .iter()
                .enumerate()
                .map(|(idx, installed_lense)| {
                    (box_idx + 1) * (idx + 1) * installed_lense.focal_length.unwrap() as usize
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn hash(input: &str) -> u8 {
    input.chars().fold(0, |acc, ch| {
        let ch_u8 = ch as u8;
        (((acc as u16 + ch_u8 as u16) * 17) % 256)
            .try_into()
            .unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, "145".to_string());
    }
}
