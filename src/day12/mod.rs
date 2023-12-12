use aoc_downloader::download_day;
use cached::proc_macro::cached;

const DAY: u32 = 12;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Vec<(String, Vec<usize>)>;

fn parse_input(input: Vec<String>) -> Input {
    input
        .iter()
        .map(|line| {
            let map = line.split(' ').next().unwrap().to_string();
            let groups = line
                .split(' ')
                .last()
                .unwrap()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            (map, groups)
        })
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

#[cached]
fn arrangement(map: Vec<u8>, groups: Vec<usize>) -> usize {
    if map.is_empty() {
        return if groups.is_empty() { 1 } else { 0 };
    }
    match map[0] {
        b'.' => return arrangement(map[1..].to_owned(), groups),
        b'?' => {
            return arrangement([&[b'.'], &map[1..]].concat(), groups.to_owned())
                + arrangement([&[b'#'], &map[1..]].concat(), groups.to_owned())
        }
        b'#' => {
            if groups.is_empty()
                || map.len() < groups[0]
                || map[..groups[0]].iter().any(|c| *c == b'.')
            {
                return 0;
            }
            if groups.len() > 1 {
                if map.len() < (groups[0] + 1) || map[groups[0]] == b'#' {
                    return 0;
                }
                return arrangement(map[(groups[0] + 1)..].to_owned(), groups[1..].to_owned());
            } else {
                return arrangement(map[(groups[0])..].to_owned(), groups[1..].to_owned());
            }
        }
        e => panic!("Unexpected character: {}", e),
    }
}

fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|(map, groups)| arrangement(map.as_bytes().to_owned(), groups.to_vec()))
        .sum()
}

fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|(map, groups)| (vec![map.clone(); 5].join("?"), groups.repeat(5)))
        .map(|(map, groups)| arrangement(map.as_bytes().to_owned(), groups.to_vec()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(6827, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(1537505634471, part2(&input));
    }
}
