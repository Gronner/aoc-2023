use itertools::Itertools;
use std::collections::HashMap;

use aoc_downloader::download_day;

const DAY: u32 = 10;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Pos = (usize, usize);
type Junction = (Pos, Pos);
type Map = HashMap<Pos, Junction>;

fn get_north(pos: Pos) -> Option<Pos> {
    pos.0.checked_sub(1).map(|y| (y, pos.1))
}

fn get_south(pos: Pos, dim: Pos) -> Option<Pos> {
    if (pos.0 + 1) < dim.0 {
        Some((pos.0 + 1, pos.1))
    } else {
        None
    }
}

fn get_west(pos: Pos) -> Option<Pos> {
    pos.1.checked_sub(1).map(|x| (pos.0, x))
}

fn get_east(pos: Pos, dim: Pos) -> Option<Pos> {
    if (pos.1 + 1) < dim.1 {
        Some((pos.0, pos.1 + 1))
    } else {
        None
    }
}

fn get_field(pos: Option<Pos>, values: &[char], input: &[String]) -> Option<Pos> {
    pos.filter(|&pos| {
        values
            .iter()
            .contains(&input[pos.0].chars().nth(pos.1).unwrap())
    })
}

fn find_start(input: &[String]) -> Option<Pos> {
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == 'S' {
                return Some((y, x));
            }
        }
    }
    None
}

fn get_start_pipe(input: &[String], start: Pos, dim: Pos) -> Option<(Pos, Junction)> {
    let north = get_north(start);
    let east = get_east(start, dim);
    let south = get_south(start, dim);
    let west = get_west(start);
    let own_north = get_field(north, &['|', '7', 'F'], input);
    let own_east = get_field(east, &['7', 'J', '-'], input);
    let own_south = get_field(south, &['|', 'J', 'L'], input);
    let own_west = get_field(west, &['-', 'L', 'F'], input);

    match (own_north, own_east, own_south, own_west) {
        (Some(n), None, Some(s), None) => Some((start, (n, s))),
        (None, Some(e), None, Some(w)) => Some((start, (e, w))),
        (Some(n), Some(e), None, None) => Some((start, (n, e))),
        (Some(n), None, None, Some(w)) => Some((start, (n, w))),
        (None, None, Some(s), Some(w)) => Some((start, (w, s))),
        (None, Some(e), Some(s), None) => Some((start, (e, s))),
        e => panic!("Found weird tile: {:?}", e),
    }
}

fn parse_input(input: Vec<String>) -> (Pos, Map) {
    let dimensions = (input.len(), input[0].len());
    let start = find_start(&input).unwrap();
    let start_junction = get_start_pipe(&input, start, dimensions);
    let map = input
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                'S' => start_junction,
                '|' => {
                    if let (Some(n), Some(s)) = (get_north((y, x)), get_south((y, x), dimensions)) {
                        Some(((y, x), (n, s)))
                    } else {
                        None
                    }
                }
                '-' => {
                    if let (Some(w), Some(e)) = (get_west((y, x)), get_east((y, x), dimensions)) {
                        Some(((y, x), (w, e)))
                    } else {
                        None
                    }
                }
                'L' => {
                    if let (Some(n), Some(e)) = (get_north((y, x)), get_east((y, x), dimensions)) {
                        Some(((y, x), (n, e)))
                    } else {
                        None
                    }
                }
                'J' => {
                    if let (Some(n), Some(w)) = (get_north((y, x)), get_west((y, x))) {
                        Some(((y, x), (n, w)))
                    } else {
                        None
                    }
                }
                '7' => {
                    if let (Some(w), Some(s)) = (get_west((y, x)), get_south((y, x), dimensions)) {
                        Some(((y, x), (w, s)))
                    } else {
                        None
                    }
                }
                'F' => {
                    if let (Some(e), Some(s)) =
                        (get_east((y, x), dimensions), get_south((y, x), dimensions))
                    {
                        Some(((y, x), (e, s)))
                    } else {
                        None
                    }
                }
                e => panic!("Unknown char encountered: {}", e),
            })
        })
        .collect();
    (start, map)
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

fn part1(input: &(Pos, Map)) -> usize {
    let (start, map) = input;
    let mut pos = map.get(start).unwrap().0;
    let mut prev_pos = *start;
    let mut steps = 0;
    while pos != *start {
        let moves = map.get(&pos).unwrap();
        pos = if moves.0 == prev_pos {
            prev_pos = pos;
            moves.1
        } else {
            prev_pos = pos;
            moves.0
        };
        steps += 1;
    }
    steps / 2 + 1
}

fn determinate(lhs: &Pos, rhs: &Pos) -> i64 {
    TryInto::<i64>::try_into(lhs.0 * rhs.1).unwrap()
        - TryInto::<i64>::try_into(lhs.1 * rhs.0).unwrap()
}

fn part2(input: &(Pos, Map)) -> i64 {
    let (start, map) = input;
    let mut pos = map.get(start).unwrap().0;
    let mut prev_pos = *start;
    let mut boundary = vec![*start, pos];
    while pos != *start {
        let moves = map.get(&pos).unwrap();
        pos = if moves.0 == prev_pos {
            prev_pos = pos;
            moves.1
        } else {
            prev_pos = pos;
            moves.0
        };
        boundary.push(pos);
    }
    // Makes windows easier, as we need start at the end again
    boundary.push(*start);
    // Apply Sholace formula, due to start trick we need to add 2 instead of 1 and deduct the
    // boundaries area
    boundary
        .windows(2)
        .map(|win| determinate(&win[0], &win[1]))
        .sum::<i64>()
        .abs()
        / 2
        + 2
        - boundary.len() as i64 / 2
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
