fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut counter = 0;
    loop {
        let hashed = md5::compute(input.to_owned() + &counter.to_string());
        if let true = &hashed[0..3].iter().all(|hex| hex == &0u8) {
            break;
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
        let result = part2("abcdef");
        assert_eq!(result, "609043".to_string());
    }
}
