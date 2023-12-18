use aoc_downloader::download_day;
use pathfinding::{matrix::Matrix, prelude::dijkstra};

const DAY: u32 = 17;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Matrix<u32>;
type Node = (((usize, usize), (isize, isize), usize), u32);

fn parse_input(input: Vec<String>) -> Input {
    Matrix::from_rows(
        input
            .iter()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .unwrap()
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

fn next_stop(
    input: &Input,
    pos: (usize, usize),
    direction: (isize, isize),
    travled: usize,
) -> Option<Node> {
    input
        .move_in_direction(pos, direction)
        .map(|new_pos| ((new_pos, direction, travled), input[new_pos]))
}

fn next_stops(
    input: &Input,
    pos: (usize, usize),
    direction: (isize, isize),
    travled: usize,
    max_len: usize,
    min_len: usize,
) -> Vec<Node> {
    let mut successor = Vec::new();
    if travled < max_len {
        successor.extend(next_stop(
            input,
            pos,
            (direction.0, direction.1),
            travled + 1,
        ));
    }
    if travled >= min_len {
        successor.extend(next_stop(input, pos, (-direction.1, -direction.0), 1));
        successor.extend(next_stop(input, pos, (direction.1, direction.0), 1));
    } else if travled == 0 {
        successor.extend(next_stop(input, pos, (1, 0), 1));
        successor.extend(next_stop(input, pos, (0, 1), 1));
    }
    successor
}

fn chocho(input: &Input, max_len: usize, min_len: usize) -> usize {
    dijkstra(
        &((0, 0), (0, 0), 0), //start, direction, traveled
        |&(pos, direction, travled)| next_stops(input, pos, direction, travled, max_len, min_len),
        |&(pos, _, _)| pos == (input.rows - 1, input.columns - 1),
    )
    .unwrap()
    .1 as usize
}

fn part1(input: &Input) -> usize {
    chocho(input, 3, 1)
}

fn part2(input: &Input) -> usize {
    chocho(input, 10, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(1155, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(1283, part2(&input));
    }
}
