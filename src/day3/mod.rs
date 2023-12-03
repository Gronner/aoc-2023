use std::collections::{HashMap, HashSet};

use aoc_downloader::download_day;

const DAY: u32 = 3;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn parse_input(input: Vec<String>) -> HashMap<Part, Vec<u32>> {
    let input = input.iter().map(|l| l.chars().collect()).collect();
    let numbers = extract_numbers(&input);
    map_parts_to_number(&input, &numbers)
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

fn get_parts(positions: &Vec<Pos>, input: &Vec<Vec<char>>) -> HashSet<Part> {
    const OFFSETS: [(isize, isize); 8] = [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut parts = HashSet::new();
    for position in positions {
        for offset in OFFSETS {
            let next_row = position.0 as isize + offset.0;
            let next_col = position.1 as isize + offset.1;
            if next_row < 0
                || next_row >= input.len() as isize
                || next_col < 0
                || next_col >= input[0].len() as isize
            {
                continue;
            }
            let symbol = input[next_row as usize][next_col as usize];
            if !(symbol.is_ascii_digit() || symbol == '.') {
                parts.insert(Part { symbol, position: (next_row as usize, next_col as usize) });
            }
        }
    }
    parts
}

type Pos = (usize, usize);

#[derive(Debug)]
struct Number {
    num: u32,
    positions: Vec<Pos>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Part {
    symbol: char,
    position: Pos,
}

fn extract_numbers(input: &Vec<Vec<char>>) -> Vec<Number> {
    let mut numbers = Vec::new();
    for (r, row) in input.iter().enumerate() {
        let mut number = String::new();
        let mut positions = Vec::new();
        for (c, col) in row.iter().enumerate() {
            if col.is_ascii_digit() {
                number.push(*col);
                positions.push((r, c));
            } else if !number.is_empty() {
                numbers.push(Number {
                    num: number.parse().unwrap(),
                    positions: positions.clone(),
                });
                number.clear();
                positions.clear();
            }
        }
        if !number.is_empty() {
            numbers.push(Number {
                num: number.parse().unwrap(),
                positions: positions.clone(),
            });
            number.clear();
            positions.clear();
        }
    }
    numbers
}

fn map_parts_to_number(
    input: &Vec<Vec<char>>,
    numbers: &Vec<Number>,
) -> HashMap<Part, Vec<u32>> {
    let mut part_mapping = HashMap::new();
    for number in numbers {
        for p in get_parts(&number.positions, input) {
            part_mapping
                .entry(p)
                .and_modify(|m: &mut Vec<u32>| m.push(number.num))
                .or_insert(vec![number.num]);
        }
    }
    part_mapping
}

fn part1(input: &HashMap<Part, Vec<u32>>) -> u32 {
    input
        .clone()
        .into_values()
        .map(|nums| nums.iter().sum::<u32>())
        .sum()
}

fn part2(input: &HashMap<Part, Vec<u32>>) -> u32 {
    input
        .keys()
        .filter(|k| k.symbol == '*')
        .map(|k| input.get(k).unwrap())
        .filter(|gear| gear.len() == 2)
        .map(|gear| gear.iter().product::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(521601, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(80694070, part2(&input));
    }
}
