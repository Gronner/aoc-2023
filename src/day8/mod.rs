use std::collections::HashMap;
use num::Integer;

use aoc_downloader::download_day;

const DAY: u32 = 8;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Map = HashMap<String, (String, String)>;
type Directions = Vec<char>;

fn parse_input(input: Vec<String>) -> (Directions, Map) {
    let re = regex!(r"([12A-Z]{3}) = \(([12A-Z]{3}), ([12A-Z]{3})\)");
    let directions = input[0].chars().collect();
    let mut map = HashMap::new();
    input[2..].iter().map(|l| re.captures(l).unwrap())
        .for_each(|cap| { map.insert(cap.get(1).unwrap().as_str().to_string(), (cap.get(2).unwrap().as_str().to_string(), cap.get(3).unwrap().as_str().to_string()));});

    (directions, map)
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

fn part1(input: &(Directions, Map)) -> usize {
    let (directions, map) = input;
    let mut pos = String::from("AAA");
    let mut steps = 0;
    while pos != "ZZZ" {
        let direction = directions[steps % input.0.len()];
        if direction == 'L' {
            pos = map.get(&pos).unwrap().0.clone()
        } else {
            pos = map.get(&pos).unwrap().1.clone()
        }
        steps += 1;
    }
    steps
}

fn part2(input: &(Directions, Map)) -> usize {
    let (directions, map) = input;
    let paths = map.keys().filter(|node| node.ends_with("A"))
        .map(|pos| {
            let mut path = Vec::new();
            path.push(pos.clone());
            let mut p = pos.clone();
            let mut steps = 0;
            while !p.ends_with("Z") {
                let direction = directions[steps % input.0.len()];
                if direction == 'L' {
                    p = map.get(&p).unwrap().0.clone();
                    path.push(p.clone());
                } else {
                    p = map.get(&p).unwrap().1.clone();
                    path.push(p.clone());
                }
                steps += 1;
            }
            path
        })
    .collect::<Vec<Vec<_>>>();
    println!("Reached");
    let cycle_lengths = paths.iter().map(|path| path.len() - 1).collect::<Vec<_>>();
    cycle_lengths.iter().skip(1).fold(cycle_lengths[0], |lcm, clen| clen.lcm(&lcm))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(22199, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(13334102464297, part2(&input));
    }
}
