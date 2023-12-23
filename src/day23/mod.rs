use std::collections::{HashMap, HashSet, VecDeque};

use aoc_downloader::download_day;
use itertools::Itertools;
use pathfinding::directed::bfs::bfs;
use pathfinding::grid::Grid;
use pathfinding::prelude::yen;

const DAY: u32 = 23;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Pos = (usize, usize);
type Slopes = HashMap<Pos, char>;
type Input = (Grid, Slopes);
type Graph = HashMap<Pos, Vec<(Pos, usize)>>;

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
                    .filter(|(_, c)| ['^', '>', '<', 'v'].contains(c))
                    .map(move |(x, c)| ((x, y), c))
            })
            .collect::<Slopes>(),
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

fn part1(input: &Input) -> usize {
    let (grid, slopes) = input;
    let start = (1_usize, 0_usize);
    let end = (grid.width - 1, grid.height - 1);
    *yen(
        &start,
        |node| {
            grid.neighbours(*node)
                .iter()
                .filter(|next| {
                    if let Some(slope) = slopes.get(next) {
                        let dir = (
                            next.0 as isize - node.0 as isize,
                            next.1 as isize - node.1 as isize,
                        );
                        matches!(
                            (dir, slope),
                            ((0, -1), '^') | ((1, 0), '>') | ((0, 1), 'v') | ((-1, 0), '<')
                        )
                    } else {
                        true
                    }
                })
                .map(|next| (*next, 1))
                .collect::<Vec<_>>()
        },
        |node| *node == end,
        1000,
    )
    .iter()
    .map(|(_, len)| len)
    .max()
    .unwrap()
}

fn prune_graph(grid: &Grid) -> Graph {
    let mut new_grid: Graph = HashMap::new();
    let start = (1_usize, 0_usize);
    let end = (grid.width - 1, grid.height - 1);

    let mut junctions = Vec::new();

    for coords in grid {
        if grid.neighbours(coords).len() > 2 {
            junctions.push(coords);
        }
    }

    junctions.push(start);
    junctions.push(end);
    let junctions = junctions;
    for pair in junctions.iter().cartesian_product(junctions.iter()) {
        if pair.0 == pair.1 {
            continue;
        }
        let mut stripped_j = junctions.clone();
        stripped_j.retain(|node| node != pair.1);
        if let Some(path) = bfs(
            pair.0,
            |node| {
                grid.neighbours(*node)
                    .iter()
                    .filter(|node| !stripped_j.contains(*node))
                    .copied()
                    .collect::<Vec<_>>()
            },
            |node| node == pair.1,
        ) {
            new_grid
                .entry(*pair.0)
                .and_modify(|vec| vec.push((*pair.1, path.len() - 1)))
                .or_insert(vec![(*pair.1, path.len() - 1)]);
        }
    }
    new_grid
}

fn part2(input: &Input) -> usize {
    let (grid, _) = input;
    let start = (1_usize, 0_usize);
    let end = (grid.width - 1, grid.height - 1);
    let new_grid = prune_graph(grid);

    let mut result = 0;
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut queue = VecDeque::new();
    queue.push_back((start, visited, 0));

    while let Some((start, visited, cost)) = queue.pop_front() {
        if start == end {
            result = result.max(cost);
            continue;
        }

        for &(next, extra) in &new_grid[&start] {
            if !visited.contains(&next) {
                let mut copy = visited.clone();
                copy.insert(next);

                queue.push_back((next, copy, cost + extra));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(2202, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(6226, part2(&input));
    }
}
