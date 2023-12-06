fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}
#[derive(Debug)]
struct Race {
    time_limit: u32,
    distance_record: u64,
}

fn part2(input: &str) -> String {
    let mut races = Vec::new();
    let mut lines_iter = input.lines();

    let time = lines_iter
        .next()
        .expect("a string of times")
        .split(':')
        .last()
        .expect("a string slice of numbers")
        .trim()
        .chars()
        .filter_map(|time_char| match time_char.to_digit(10) {
            Some(num) => Some(num.to_string()),
            None => None,
        })
        .collect::<String>()
        .parse::<u32>()
        .expect("a single number");

    let distance = lines_iter
        .next()
        .expect("a string of times")
        .split(':')
        .last()
        .expect("a string slice of numbers")
        .trim()
        .chars()
        .filter_map(|time_char| match time_char.to_digit(10) {
            Some(num) => Some(num.to_string()),
            None => None,
        })
        .collect::<String>()
        .parse::<u64>()
        .expect("a single number.");

    races.push(Race {
        time_limit: time,
        distance_record: distance,
    });

    dbg!(&races);
    fn mim_charge_time(time_limit: u32, goal_distance: u64) -> u32 {
        // println!("Min Charge Func: limit:{time_limit} goal:{goal_distance}");
        let raw_min =
            -((goal_distance as f32 - (time_limit as f32 / 2 as f32).powf(2.0)) / -1 as f32).sqrt()
                + (time_limit as f32 / 2.0);
        // dbg!(raw_min);
        unsafe { raw_min.ceil().to_int_unchecked::<u32>() }
    }

    fn count_combos(time_limit: u32, min_charge_time: u32) -> u32 {
        // println!("Given time limit of {time_limit} and min charge time of {min_charge_time}");
        (time_limit + 1) - (min_charge_time * 2)
    }

    races
        .iter()
        .map(|race| {
            let min_time = mim_charge_time(race.time_limit, race.distance_record + 1);
            count_combos(race.time_limit, min_time)
        })
        .sum::<u32>()
        .to_string()

    // "0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, "71503".to_string());
    }
}
