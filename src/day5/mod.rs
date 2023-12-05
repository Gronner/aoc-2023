use std::{str::FromStr, fmt::Display};
use itertools::Itertools;

use aoc_downloader::download_day;

const DAY: u32 = 5;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn extract_mapping_to_(header: &str, kind: Mapping, input: &Vec<String>) -> Vec<Range> {
    input.iter()
        .skip_while(|line| line != &header)
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| Range::from_string(line, kind))
        .collect()
}

fn parse_input(input: Vec<String>) -> (Vec<Seed>, Vec<Range>) {
    let seeds: Vec<Seed> = input[0].split(' ').skip(1).map(|s| Seed::from_str(s).unwrap()).collect();

    let mut mapping = extract_mapping_to_("seed-to-soil map:", Mapping::ToSoil, &input);
    mapping.append(&mut extract_mapping_to_("soil-to-fertilizer map:", Mapping::ToFertilizer, &input));
    mapping.append(&mut extract_mapping_to_("fertilizer-to-water map:", Mapping::ToWater, &input));
    mapping.append(&mut extract_mapping_to_("water-to-light map:", Mapping::ToLight, &input));
    mapping.append(&mut extract_mapping_to_("light-to-temperature map:", Mapping::ToTemperature, &input));
    mapping.append(&mut extract_mapping_to_("temperature-to-humidity map:", Mapping::ToHumidity, &input));
    mapping.append(&mut extract_mapping_to_("humidity-to-location map:", Mapping::ToLocation, &input));

    (seeds, mapping)
}

#[derive(Clone, Copy, PartialOrd, Ord,PartialEq, Eq, Hash, Debug)]
struct Seed(usize);

impl Display for Seed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Seed {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Seed(s.parse().unwrap_or_else(|_| panic!("Can't parse: {}", s))))
    }
}

#[derive(Debug)]
struct Range {
    pub destination: usize,
    pub source: usize,
    pub length: usize,
    pub kind: Mapping,
}

impl Range {
    fn seed_in_range(&self, seed: &Seed) -> bool {
        seed.0 >= self.source && seed.0 < (self.source + self.length)
    }

    fn map_seed(&self, s: &Seed) -> Seed {
        let offset = s.0 - self.source;
        Seed(self.destination + offset)
    }

    fn from_string(s: &str, kind: Mapping) -> Self {
        let values: Vec<usize> = s.split(' ')
            .map(|num| num.parse().unwrap_or_else(|_| panic!("Can't parse: {}", num)))
            .collect();
        Range {
            destination: values[0],
            source: values[1],
            length: values[2],
            kind,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Mapping {
    ToSoil,
    ToFertilizer,
    ToWater,
    ToLight,
    ToTemperature,
    ToHumidity,
    ToLocation,
}



pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!(
        "Running day {}:\n\tPart1 {}\n\tPart2 {}",
        DAY,
        part1(&input),
        part2(&input)
    );
}

fn apply_map(seeds: &Vec<Seed>, mappings: &Vec<Range>, kind: Mapping) -> Vec<Seed> {
    seeds.iter()
        .map(|s| {
            for mapping in mappings.iter() {
                if mapping.kind == kind && mapping.seed_in_range(s) {
                    return mapping.map_seed(s)
                }
            }
            *s
        })
        .collect()
}

fn part1(input: &(Vec<Seed>, Vec<Range>)) -> Seed {
    let (seeds, mappings) = input;
    let seeds: Vec<Seed> = apply_map(seeds, mappings, Mapping::ToSoil);
    let seeds: Vec<Seed> = apply_map(&seeds, mappings, Mapping::ToFertilizer);
    let seeds: Vec<Seed> = apply_map(&seeds, mappings, Mapping::ToWater);
    let seeds: Vec<Seed> = apply_map(&seeds, mappings, Mapping::ToLight);
    let seeds: Vec<Seed> = apply_map(&seeds, mappings, Mapping::ToTemperature);
    let seeds: Vec<Seed> = apply_map(&seeds, mappings, Mapping::ToHumidity);
    let seeds: Vec<Seed> = apply_map(&seeds, mappings, Mapping::ToLocation);

    *seeds.iter().min().unwrap()
}

fn part2(input: &(Vec<Seed>, Vec<Range>)) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(2486, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(87984, part2(&input));
    }
}
