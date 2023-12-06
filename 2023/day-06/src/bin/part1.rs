fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}
#[derive(Debug)]
struct Race {
    time_limit: u32,
    distance_record: u32,
}

fn part1(input: &str) -> String {
    let mut races = Vec::new();
    let mut lines_iter = input.lines();

    let times = lines_iter
        .next()
        .expect("a string of times")
        .split(':')
        .last()
        .expect("a string slice of numbers")
        .trim()
        .split_ascii_whitespace()
        .filter_map(|time_str| match time_str.parse::<u32>() {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .collect::<Vec<u32>>();

    let distances = lines_iter
        .next()
        .expect("a string of times")
        .split(':')
        .last()
        .expect("a string slice of numbers")
        .trim()
        .split_ascii_whitespace()
        .filter_map(|time_str| match time_str.parse::<u32>() {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .collect::<Vec<u32>>();

    for (idx, time) in times.iter().enumerate() {
        races.push(Race {
            time_limit: *time,
            distance_record: distances[idx],
        })
    }

    fn mim_charge_time(time_limit: u32, goal_distance: u32) -> u32 {
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
        .product::<u32>()
        .to_string()

    // "0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, "288".to_string());
    }
}
