use multimap::MultiMap;
use std::{hash::Hash, str::FromStr};

use aoc_downloader::download_day;

const DAY: u32 = 15;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Vec<Lense>;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
}

#[derive(Debug)]
struct Lense {
    pub label: String,
    pub focal_length: Option<u64>,
    pub operation: Operation,
    pub hash: u64,
}

impl Eq for Lense {}

impl PartialEq for Lense {
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

impl Hash for Lense {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state)
    }
}

fn hash(s: &str) -> u64 {
    s.chars().fold(0, |acc, c| ((acc + c as u64) * 17) % 256)
}

impl FromStr for Lense {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hash = hash(s);
        let (label, focal_length, operation) = if s.contains('-') {
            let mut split = s.split('-');
            (
                split.next().unwrap().to_string(),
                split.next().and_then(|s| s.parse().ok()),
                Operation::Sub,
            )
        } else {
            let mut split = s.split('=');
            (
                split.next().unwrap().to_string(),
                split.next().and_then(|s| s.parse().ok()),
                Operation::Add,
            )
        };
        Ok(Lense {
            label,
            focal_length,
            operation,
            hash,
        })
    }
}

fn parse_input(input: Vec<String>) -> Input {
    input
        .iter()
        .flat_map(|row| {
            row.split(',')
                .map(|s| Lense::from_str(s).unwrap())
                .collect::<Vec<Lense>>()
        })
        .collect()
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

fn part1(input: &Input) -> u64 {
    input.iter().map(|lens| lens.hash).sum()
}

fn part2(input: &Input) -> u64 {
    let mut boxes: MultiMap<u64, (String, u64)> = MultiMap::new();
    for lense in input {
        match lense.operation {
            Operation::Add => {
                if let Some(idx) = boxes
                    .get_vec(&hash(&lense.label))
                    .and_then(|vec| vec.iter().position(|val| val.0 == lense.label))
                {
                    boxes.get_vec_mut(&hash(&lense.label)).map(|vec| {
                        vec[idx] = (lense.label.clone(), lense.focal_length.unwrap());
                        Some(())
                    });
                } else {
                    boxes.insert(
                        hash(&lense.label),
                        (lense.label.clone(), lense.focal_length.unwrap()),
                    );
                }
            }
            Operation::Sub => {
                boxes.get_vec_mut(&hash(&lense.label)).map(|vec| {
                    vec.retain(|val| val.0 != lense.label);
                    Some(())
                });
            }
        }
    }
    let mut sum = 0;
    for (key, vals) in boxes {
        for (i, val) in vals.iter().enumerate() {
            sum += (key + 1) * (i as u64 + 1) * val.1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(512797, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(262454, part2(&input));
    }
}
