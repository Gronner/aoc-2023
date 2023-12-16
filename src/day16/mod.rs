use std::collections::{HashMap, HashSet};

use aoc_downloader::download_day;
use itertools::Itertools;
use num::complex::Complex;

const DAY: u32 = 16;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Pos = Complex<isize>;
type Input = (HashMap<Complex<isize>, Part>, isize, isize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Part {
    AcMirror,
    BdMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl From<char> for Part {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            '\\' => Self::AcMirror,
            '/' => Self::BdMirror,
            e => panic!("Unexpected character: {}", e),
        }
    }
}

fn parse_input(input: Vec<String>) -> Input {
    (
        input
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(move |(x, c)| (Complex::new(y as isize, x as isize), Part::from(c)))
            })
            .collect(),
        input.len() as isize,
        input[0].len() as isize,
    )
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

fn travel(
    start: (Complex<isize>, Complex<isize>),
    contraption: &HashMap<Complex<isize>, Part>,
    y_max: isize,
    x_max: isize,
) -> usize {
    let mut rays = vec![start];
    let mut visited = HashSet::new();
    while let Some((pos, dir)) = rays.pop() {
        if visited.contains(&(pos, dir)) {
            continue;
        }
        visited.insert((pos, dir));

        let next_pos = pos + dir;
        if next_pos.re < 0 || next_pos.re >= y_max || next_pos.im < 0 || next_pos.im >= x_max {
            continue;
        }
        if let Some(part) = contraption.get(&next_pos) {
            match part {
                Part::AcMirror => {
                    rays.push((next_pos, (dir * Complex::new(0, -1)).conj()));
                }
                Part::BdMirror => {
                    rays.push((next_pos, (dir * Complex::new(0, 1)).conj()));
                }
                Part::VerticalSplitter if dir.im != 0 => {
                    rays.push((next_pos, Complex::new(1, 0)));
                    rays.push((next_pos, Complex::new(-1, 0)))
                }
                Part::HorizontalSplitter if dir.re != 0 => {
                    rays.push((next_pos, Complex::new(0, 1)));
                    rays.push((next_pos, Complex::new(0, -1)))
                }
                _ => rays.push((next_pos, dir)),
            }
        } else {
            rays.push((next_pos, dir));
        }
    }

    visited.iter().map(|(pos, _)| pos).unique().count() - 1
}

fn part1(input: &Input) -> usize {
    let (contraption, y_max, x_max) = input;
    travel(
        (Complex::new(0, -1), Complex::new(0, 1)),
        contraption,
        *y_max,
        *x_max,
    )
}

fn part2(input: &Input) -> usize {
    let (contraption, y_max, x_max) = input;
    let mut starts = (0..*y_max)
        .flat_map(|y| {
            vec![
                (Complex::new(y, -1), Complex::new(0, 1)),
                (Complex::new(y, *x_max), Complex::new(0, -1)),
            ]
        })
        .collect::<Vec<(Pos, Pos)>>();
    starts.append(
        &mut (0..*x_max)
            .flat_map(|x| {
                vec![
                    (Complex::new(-1, x), Complex::new(1, 0)),
                    (Complex::new(*y_max, x), Complex::new(-1, 0)),
                ]
            })
            .collect::<Vec<(Pos, Pos)>>(),
    );

    starts
        .iter()
        .map(|start| travel(*start, contraption, *y_max, *x_max))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(7210, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(7673, part2(&input));
    }
}
