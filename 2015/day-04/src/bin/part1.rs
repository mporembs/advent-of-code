fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut counter = 0;
    loop {
        let hashed = md5::compute(input.to_owned() + &counter.to_string());
        // println!("{:?}", hashed);
        let prefix = &hashed[0..3];
        if prefix[0].eq(&0u8) && prefix[1].eq(&0u8) {
            match (0..=15).contains(&prefix[2]) {
                true => {
                    break;
                }

                false => (),
            }
        }
        counter += 1;
    }
    counter.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("abcdef");
        assert_eq!(result, "609043".to_string());
    }
}
