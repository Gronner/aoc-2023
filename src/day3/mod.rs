use std::collections::{HashSet, HashMap};

use aoc_downloader::download_day;

const DAY: u32 = 3;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn parse_input(input: Vec<String>) -> Vec<Vec<char>> {
    input.iter()
        .map(|l| l.chars().collect())
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

fn get_neighbours(positions: Vec<Pos>, input: &Vec<Vec<char>>) -> Vec<(char, Pos)> {
    const OFFSETS: [(isize, isize); 8] = [(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)];
    let mut neighbours_positions = HashSet::new();
    for position in positions {
        for offset in OFFSETS {
            let next_row = position.0 as isize + offset.0;
            let next_col = position.1 as isize + offset.1;
            if next_row < 0 || next_row >= input.len() as isize || next_col < 0 || next_col >= input[0].len() as isize {
                continue;
            }
            neighbours_positions.insert((next_row as usize, next_col as usize));
        }
    }
    let mut neighbours = Vec::new();
    for pos in neighbours_positions {
        neighbours.push((input[pos.0][pos.1], (pos.0, pos.1)));
    }
    neighbours
}

type Pos = (usize, usize);

#[derive(Debug)]
struct Number {
    num: u32,
    positions: Vec<Pos>
}

fn part1(input: &Vec<Vec<char>>) -> u32 {
    let mut numbers = Vec::new();
    for (r, row) in input.iter().enumerate() {
        let mut number = String::new();
        let mut positions = Vec::new();
        for (c, col) in row.iter().enumerate() {
            if col.is_ascii_digit() {
                number.push(*col);
                positions.push((r, c));
            } else {
                if !number.is_empty() {
                    numbers.push(Number { num: number.parse().unwrap(), positions: positions.clone() });
                    number.clear();
                    positions.clear();
                }
            }
        }
        if !number.is_empty() {
            numbers.push(Number { num: number.parse().unwrap(), positions: positions.clone() });
            number.clear();
            positions.clear();
        }
    }

    println!("{:?}", numbers);

    let mut part_mapping = HashMap::new();
    for number in numbers {
        for n in get_neighbours(number.positions, input) {
            if !(n.0.is_ascii_digit() || n.0 == '.') {
                part_mapping.entry(n).and_modify(|m: &mut Vec<u32> | m.push(number.num)).or_insert(vec![number.num]);
            }
        }
    }

    println!("{:?}", part_mapping);
    part_mapping.into_values().map(|nums| nums.iter().sum::<u32>()).sum()
}

fn part2(input: &Vec<Vec<char>>) -> u32 {
    let mut numbers = Vec::new();
    for (r, row) in input.iter().enumerate() {
        let mut number = String::new();
        let mut positions = Vec::new();
        for (c, col) in row.iter().enumerate() {
            if col.is_ascii_digit() {
                number.push(*col);
                positions.push((r, c));
            } else {
                if !number.is_empty() {
                    numbers.push(Number { num: number.parse().unwrap(), positions: positions.clone() });
                    number.clear();
                    positions.clear();
                }
            }
        }
        if !number.is_empty() {
            numbers.push(Number { num: number.parse().unwrap(), positions: positions.clone() });
            number.clear();
            positions.clear();
        }
    }

    println!("{:?}", numbers);

    let mut part_mapping = HashMap::new();
    for number in numbers {
        for n in get_neighbours(number.positions, input) {
            if !(n.0.is_ascii_digit() || n.0 == '.') {
                part_mapping.entry(n).and_modify(|m: &mut Vec<u32> | m.push(number.num)).or_insert(vec![number.num]);
            }
        }
    }

    println!("{:?}", part_mapping);
    part_mapping.keys()
        .filter(|k| k.0 == '*')
        .map(|k| part_mapping.get(k).unwrap())
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
        assert_eq!(2486, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(87984, part2(&input));
    }
}
