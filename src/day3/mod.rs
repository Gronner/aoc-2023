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

fn read_schematic(schematic: &Vec<Vec<char>>) -> (Vec<Number>, Vec<Part>) {
    let mut numbers = Vec::new();
    let mut parts = Vec::new();

    for (r, row) in schematic.iter().enumerate() {
        let mut number = String::new();
        let mut positions = HashSet::new();
        for (c, col) in row.iter().enumerate() {
            if col.is_ascii_digit() {
                number.push(*col);
                positions.insert((r, c));
            } else {
                if *col != '.' {
                    parts.push(Part { symbol: *col, position: (r, c) });
                }
                if !number.is_empty() {
                    numbers.push(Number {num: number.parse().unwrap(), positions: positions.clone()});
                    number.clear();
                    positions.clear();
                }
            }
        }
        if !number.is_empty() {
            numbers.push(Number {num: number.parse().unwrap(), positions: positions.clone()});
            number.clear();
            positions.clear();
        }
    }

    (numbers, parts)
}

fn relate_part_numbers(numbers: &Vec<Number>, parts: &Vec<Part>, dimensions: Pos) -> HashMap<Part, Vec<u32>> {
    let mut part_mapping =  HashMap::new();
    for part in parts {
        for number in numbers {
            if part.is_part_number(number, dimensions) {
                part_mapping
                    .entry(*part)
                    .and_modify(|m: &mut Vec<u32>| m.push(number.num))
                    .or_insert(vec![number.num]);
            }
        }
    }
    part_mapping
}

fn parse_input(input: Vec<String>) -> HashMap<Part, Vec<u32>> {
    let input = input.iter().map(|l| l.chars().collect()).collect();
    let (numbers, parts) = read_schematic(&input);
    relate_part_numbers(&numbers, &parts, (input.len(), input[0].len()))
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
type Pos = (usize, usize);

#[derive(Debug)]
struct Number {
    pub num: u32,
    pub positions: HashSet<Pos>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Part {
    symbol: char,
    position: Pos,
}

impl Part { 
    fn is_part_number(&self, number: &Number, dimensions: Pos) -> bool {
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
        let mut adjacent = HashSet::new();
        for offset in OFFSETS {
            let next_row = self.position.0.checked_add_signed(offset.0);
            let next_col = self.position.1.checked_add_signed(offset.1);
            if next_row.is_some_and(|r| r < dimensions.0) && next_col.is_some_and(|c| c < dimensions.1) {
                adjacent.insert((next_row.unwrap(), next_col.unwrap()));
            }
        }
        !number.positions.is_disjoint(&adjacent)
    }
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
