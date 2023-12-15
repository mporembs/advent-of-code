fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input
        .split(',')
        .map(|instruction| hash(instruction) as usize)
        .sum::<usize>()
        .to_string()
    // dbg!(test);
    // "0".to_string()
}

fn hash(input: &str) -> u8 {
    input.chars().fold(0, |acc, ch| {
        // todo!();
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
        let result = part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, "1320".to_string());
    }
}
