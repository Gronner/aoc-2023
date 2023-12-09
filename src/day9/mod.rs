use aoc_downloader::download_day;
use itertools::Itertools;

const DAY: u32 = 9;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn parse_input(input: Vec<String>) -> Vec<Vec<i64>> {
    input
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<_>>>()
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

trait Derive {
    fn derive(&self) -> Vec<i64>;
}

impl Derive for &Vec<i64> {
    fn derive(&self) -> Vec<i64> {
        self.iter().map_windows(|[&a, &b]| b - a).collect_vec()
    }
}

fn part1(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|sequence| {
            let mut stack: Vec<Vec<i64>> = vec![sequence.clone()];
            while stack.last().unwrap().iter().any(|n| *n != 0) {
                stack.push(stack.last().unwrap().derive());
            }
            stack
                .iter()
                .rev()
                .inspect(|seq| println!("{:?}", seq))
                .fold(0, |acc, sequence| acc + sequence.last().unwrap())
        })
        .sum()
}

fn part2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|sequence| {
            let mut stack: Vec<Vec<i64>> = vec![sequence.clone()];
            while stack.last().unwrap().iter().any(|n| *n != 0) {
                stack.push(stack.last().unwrap().derive());
            }
            stack
                .iter()
                .rev()
                .inspect(|seq| println!("{:?}", seq))
                .fold(0, |acc, sequence| {
                    println!("{acc}");
                    sequence.first().unwrap() - acc
                })
        })
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
