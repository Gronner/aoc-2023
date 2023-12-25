use aoc_downloader::download_day;
use graphrs::{algorithms::community::louvain, Edge, Graph, GraphSpecs};

const DAY: u32 = 25;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Graph<String, ()>;

fn parse_input(input: Vec<String>) -> Input {
    let mut graph = Graph::<String, ()>::new(GraphSpecs::undirected_create_missing());
    input
        .iter()
        .map(|line| {
            let splits = line.split_once(": ").unwrap();
            let from = splits.0.to_string();
            let tos = splits
                .1
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            (from, tos)
        })
        .for_each(|(from, tos)| {
            tos.iter().for_each(|to| {
                graph.add_edge(Edge::new(from.clone(), to.clone())).unwrap();
            });
        });
    graph
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
    let min_cut = louvain::louvain_partitions(input, false, Some(0.), Some(4.), None).unwrap();
    min_cut[0].iter().map(|node| node.len()).product()
}

fn part2(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(598120, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(0, part2(&input));
    }
}
