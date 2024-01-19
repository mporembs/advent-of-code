use day_02::gift::Gift;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    multi::separated_list0,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let (_, presents) = parse_gifts(input).unwrap();
    let sqft = presents.iter().fold(0, |acc, gift| {
        acc + gift.surface_area() + gift.smallest_side()
    });

    sqft.to_string()
}

fn parse_gift(input: &str) -> IResult<&str, Gift> {
    let (input, dimensions) = separated_list0(tag("x"), u32)(input)?;

    Ok((
        input,
        Gift::new(dimensions[0], dimensions[1], dimensions[2]),
    ))
}

fn parse_gifts(input: &str) -> IResult<&str, Vec<Gift>> {
    separated_list0(newline, parse_gift)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("2x3x4");
        assert_eq!(result, "58".to_string());
    }
}
