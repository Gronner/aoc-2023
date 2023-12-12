use itertools::Itertools;

use aoc_downloader::download_day;

const DAY: u32 = 11;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = (Vec<Pos>, XGaps, YGaps);
type Pos = (usize, usize);
type XGaps = Vec<usize>;
type YGaps = Vec<usize>;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn parse_input(input: Vec<String>) -> Input {
    let positions = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (y, x))
        })
        .collect();

    let ygaps = input
        .iter()
        .enumerate()
        .filter(|(_, row)| !row.contains('#'))
        .map(|(y, _)| y)
        .collect();

    let input_t = transpose(
        input
            .iter()
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<_>>>(),
    );
    let input_t = input_t
        .iter()
        .map(|vec| vec.iter().collect::<String>())
        .collect::<Vec<String>>();

    let xgaps = input_t
        .iter()
        .enumerate()
        .filter(|(_, row)| !row.contains('#'))
        .map(|(y, _)| y)
        .collect();

    (positions, ygaps, xgaps)
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

fn manhatten_distance(pos_a: &Pos, pos_b: &Pos) -> usize {
    (pos_a.0 as isize - pos_b.0 as isize).unsigned_abs()
        + (pos_a.1 as isize - pos_b.1 as isize).unsigned_abs()
}

fn part1(input: &Input) -> usize {
    let (positions, ygaps, xgaps) = input;
    println!("{:?}", xgaps);
    println!("{:?}", ygaps);
    let _distance_sum = 0;
    let positions = positions
        .iter()
        .map(|(y, x)| {
            (
                y + ygaps.iter().filter(|ygap| y > ygap).count(),
                x + xgaps.iter().filter(|xgap| x > xgap).count(),
            )
        })
        .collect::<Vec<Pos>>();
    positions
        .iter()
        .combinations(2)
        .map(|vec| manhatten_distance(vec[0], vec[1]))
        .sum()
}

fn part2(input: &Input) -> usize {
    let (positions, ygaps, xgaps) = input;
    println!("{:?}", xgaps);
    println!("{:?}", ygaps);
    let _distance_sum = 0;
    let positions = positions
        .iter()
        .map(|(y, x)| {
            (
                y + ygaps.iter().filter(|ygap| y > ygap).count() * 999_999,
                x + xgaps.iter().filter(|xgap| x > xgap).count() * 999_999,
            )
        })
        .collect::<Vec<Pos>>();
    positions
        .iter()
        .combinations(2)
        .map(|vec| manhatten_distance(vec[0], vec[1]))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(9686930, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(630728425490, part2(&input));
    }
}
