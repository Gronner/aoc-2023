use std::collections::HashMap;

use aoc_downloader::download_day;
use num::Integer;
use pathfinding::{directed::bfs::bfs_reach, grid::Grid};

const DAY: u32 = 21;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Pos = (usize, usize);
type Input = (Grid, Pos);

fn parse_input(input: Vec<String>) -> Input {
    (
        input
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '#')
                    .map(move |(x, _)| (x, y))
            })
            .collect::<Grid>(),
        input
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == 'S')
                    .map(move |(x, _)| (x, y))
            })
            .next()
            .unwrap(),
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

fn find_n_reachable(steps: usize, grid: &Grid, start: &Pos) -> usize {
    bfs_reach((*start, 0), |node| {
        if node.1 > steps {
            vec![]
        } else {
            grid.neighbours(node.0)
                .iter()
                .map(|n| (*n, node.1 + 1))
                .collect()
        }
    })
    .collect::<Vec<_>>()
    .iter()
    .filter(|node| node.1 == steps)
    .count()
}

fn part1(input: &Input) -> usize {
    let (grid, start) = input.clone();
    find_n_reachable(64, &grid, &start)
}

fn bfs(grid: &Grid, start: &Pos) -> HashMap<Pos, usize> {
    let mut visited = HashMap::new();
    let _ = bfs_reach((*start, 0), |node| {
        if visited.contains_key(&node.0) {
            vec![]
        } else {
            visited.insert(node.0, node.1);
            grid.neighbours(node.0)
                .iter()
                .filter(|pos| !visited.contains_key(*pos))
                .map(|pos| (*pos, node.1 + 1))
                .collect()
        }
    })
    .map(|(pos, _)| pos)
    .collect::<Grid>();

    visited
}

fn part2(input: &Input) -> usize {
    // Reachable fields are a rombus - any rocks for an odd step count all with an odd step count +
    // corners for an even step count all with an even step count + corners
    const MAX_STEPS: usize = 26501365;
    const CENTER_TO_EDGE: usize = 65;

    let (grid, start) = input.clone();

    let boardsize = grid.width;
    let nodes = bfs(&grid, &start);

    let even_corners = nodes
        .iter()
        .filter(|(_, steps)| steps.is_even() && **steps > CENTER_TO_EDGE)
        .count();
    let odd_corners = nodes
        .iter()
        .filter(|(_, steps)| steps.is_odd() && **steps > CENTER_TO_EDGE)
        .count();

    let even = nodes.iter().filter(|(_, steps)| steps.is_even()).count();
    let odd = nodes.iter().filter(|(_, steps)| steps.is_odd()).count();

    let input_tiles = (MAX_STEPS - (boardsize / 2)) / boardsize;

    (input_tiles + 1) * (input_tiles + 1) * odd + (input_tiles * input_tiles) * even
        - (input_tiles + 1) * odd_corners
        + input_tiles * even_corners
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(108813, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(104533, part2(&input));
    }
}
