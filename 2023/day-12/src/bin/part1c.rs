use itertools::Itertools;

// struct Row<'a> {
//     springs: &'a str,
//     group_lengths: dyn Iterator<Item = usize>,
// }

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let _spring_records = input
        .lines()
        .map(|line| {
            let (spring, counts) = line.split_once(' ').unwrap();
            let counts = counts.split(',').map(|num| num.parse::<usize>().unwrap());
            solve(spring, counts)
        })
        .sum::<usize>();

    fn solve(spring: &str, counts: impl Iterator<Item = usize>) -> usize {
        let counts = counts.collect_vec();

        // Add '.' to the beginning of the row. Not sure why yet
        let spring = format!(".{}", spring.trim_end_matches('.'));
        let spring = spring.chars().collect_vec();

        // Create a vec of all 0's, as long as our NEW row length
        let mut dp = vec![0; spring.len() + 1];

        // Set the added '.' equal to 1
        dp[0] = 1;

        for (i, _) in spring.iter().take_while(|&&c| c != '#').enumerate() {
            dp[i + 1] = 1;
        }
        dbg!(&dp);
        for count in counts {
            let mut n_dp = vec![0; spring.len() + 1];
            let mut chunk = 0;

            for (i, &c) in spring.iter().enumerate() {
                if c != '.' {
                    chunk += 1;
                } else {
                    chunk = 0;
                }

                if c != '#' {
                    n_dp[i + 1] += n_dp[i];
                }

                if chunk >= count && spring[i - count] != '#' {
                    n_dp[i + 1] += dp[i - count];
                }
            }

            dp = n_dp;
        }

        *dp.last().unwrap()
    }
    "0".to_string()
    // spring_records.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
####.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(result, "21".to_string());
    }
}
