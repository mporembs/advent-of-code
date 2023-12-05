use std::ops::Range;
use std::str::FromStr;
use strum_macros::EnumString;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Clone, Copy, EnumString)]
#[strum(ascii_case_insensitive)]
enum Material {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
}

#[derive(Debug)]

struct ConversionMap {
    input: Material,
    dest_range: Range<u64>,
    source_range: Range<u64>,
}

fn part1(input: &str) -> String {
    let mut converters: Vec<ConversionMap> = Vec::new();
    let mut section_iter = input.split_terminator("\n\n").map(|section| section.trim());

    // dbg!(section_iter.clone().count());

    let seeds_numbers = section_iter
        .next()
        .expect("a string")
        .split(' ')
        .filter_map(|sub_string| match sub_string.parse::<u64>() {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .collect::<Vec<u64>>();
    // dbg!(seeds_numbers);

    for (_, section) in section_iter.enumerate() {
        // println!("{idx}:{section}");
        let mut sub_section_line_strs = section.lines();
        let title_line = sub_section_line_strs.next().unwrap();

        let section_input: Material =
            Material::from_str(title_line.split('-').nth(0).expect("a material string.")).unwrap();
        sub_section_line_strs.for_each(|map_line| {
            let range_length = map_line
                .split_ascii_whitespace()
                .nth(2)
                .expect("a string")
                .parse::<u64>()
                .expect("a number");
            let new_map = ConversionMap {
                input: section_input,
                dest_range: Range {
                    start: (map_line
                        .split_ascii_whitespace()
                        .nth(0)
                        .expect("a string")
                        .parse::<u64>()
                        .expect("a number")),
                    end: (map_line
                        .split_ascii_whitespace()
                        .nth(0)
                        .expect("a string")
                        .parse::<u64>()
                        .expect("a number")
                        + range_length),
                },
                source_range: Range {
                    start: (map_line
                        .split_ascii_whitespace()
                        .nth(1)
                        .expect("a string")
                        .parse::<u64>()
                        .expect("a number")),
                    end: (map_line
                        .split_ascii_whitespace()
                        .nth(1)
                        .expect("a string")
                        .parse::<u64>()
                        .expect("a number")
                        + range_length),
                },
            };
            converters.push(new_map);
        });
        // dbg!(&converters);
    }

    let locations = seeds_numbers
        .iter()
        .map(|seed_number| {
            let soil_number: u64 = {
                let converted = converters
                    .iter()
                    .filter(|converter| converter.input == Material::Seed)
                    .map(
                        |converter| match converter.source_range.contains(seed_number) {
                            true => {
                                let source_index = seed_number - converter.source_range.start;
                                converter.dest_range.start + source_index
                            }
                            false => 0,
                        },
                    )
                    .sum();
                match converted > 0 as u64 {
                    true => converted,
                    false => *seed_number,
                }
            };
            let fertilizer_number: u64 = {
                let converted = converters
                    .iter()
                    .filter(|converter| converter.input == Material::Soil)
                    .map(
                        |converter| match converter.source_range.contains(&soil_number) {
                            true => {
                                let source_index = &soil_number - converter.source_range.start;
                                converter.dest_range.start + source_index
                            }
                            false => 0,
                        },
                    )
                    .sum();
                match converted > 0 as u64 {
                    true => converted,
                    false => soil_number,
                }
            };
            let water_number: u64 = {
                let converted = converters
                    .iter()
                    .filter(|converter| converter.input == Material::Fertilizer)
                    .map(
                        |converter| match converter.source_range.contains(&fertilizer_number) {
                            true => {
                                let source_index =
                                    &fertilizer_number - converter.source_range.start;
                                converter.dest_range.start + source_index
                            }
                            false => 0,
                        },
                    )
                    .sum();
                match converted > 0 as u64 {
                    true => converted,
                    false => *&fertilizer_number,
                }
            };
            let light_number: u64 = {
                let converted = converters
                    .iter()
                    .filter(|converter| converter.input == Material::Water)
                    .map(
                        |converter| match converter.source_range.contains(&water_number) {
                            true => {
                                let source_index = &water_number - converter.source_range.start;
                                converter.dest_range.start + source_index
                            }
                            false => 0,
                        },
                    )
                    .sum();
                match converted > 0 as u64 {
                    true => converted,
                    false => *&water_number,
                }
            };
            let temperature_number: u64 = {
                let converted = converters
                    .iter()
                    .filter(|converter| converter.input == Material::Light)
                    .map(
                        |converter| match converter.source_range.contains(&light_number) {
                            true => {
                                let source_index = &light_number - converter.source_range.start;
                                converter.dest_range.start + source_index
                            }
                            false => 0,
                        },
                    )
                    .sum();
                match converted > 0 as u64 {
                    true => converted,
                    false => *&light_number,
                }
            };
            let humidity_number: u64 = {
                let converted = converters
                    .iter()
                    .filter(|converter| converter.input == Material::Temperature)
                    .map(
                        |converter| match converter.source_range.contains(&temperature_number) {
                            true => {
                                let source_index =
                                    &temperature_number - converter.source_range.start;
                                converter.dest_range.start + source_index
                            }
                            false => 0,
                        },
                    )
                    .sum();
                match converted > 0 as u64 {
                    true => converted,
                    false => *&temperature_number,
                }
            };
            let location_number: u64 = {
                let converted = converters
                    .iter()
                    .filter(|converter| converter.input == Material::Humidity)
                    .map(
                        |converter| match converter.source_range.contains(&humidity_number) {
                            true => {
                                let source_index = &humidity_number - converter.source_range.start;
                                converter.dest_range.start + source_index
                            }
                            false => 0,
                        },
                    )
                    .sum();
                match converted > 0 as u64 {
                    true => converted,
                    false => *&humidity_number,
                }
            };

            // println!("Seed {seed_number}: {soil_number}, {fertilizer_number}, {water_number}, {light_number}, {temperature_number}, {humidity_number}, {location_number}");
            location_number
        })
        .collect::<Vec<u64>>();
    // dbg!(locations);
    locations.iter().min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, "35".to_string());
    }
}
