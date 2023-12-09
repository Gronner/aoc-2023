use num::Integer;
use std::collections::HashMap;

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
    let re = regex!(r"(\w{3}) = \((\w{3}), (\w{3})\)");
    let directions = input[0].chars().collect();
    let mut map = HashMap::new();
    input[2..]
        .iter()
        .map(|l| re.captures(l).unwrap())
        .for_each(|cap| {
            map.insert(
                cap.get(1).unwrap().as_str().to_string(),
                (
                    cap.get(2).unwrap().as_str().to_string(),
                    cap.get(3).unwrap().as_str().to_string(),
                ),
            );
        });

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

fn count_steps(start: &str, goal: fn(&str) -> bool, map: &Map, directions: &Directions) -> usize {
    let mut pos = start.to_string();
    let mut steps = 0;
    while goal(&pos) {
        let direction = directions[steps % directions.len()];
        pos = match direction {
            'L' => map.get(&pos).unwrap().0.clone(),
            'R' => map.get(&pos).unwrap().1.clone(),
            c => panic!("Unexpected character: {}", c),
        };
        steps += 1;
    }
    steps
}

fn part1(input: &(Directions, Map)) -> usize {
    let (directions, map) = input;
    count_steps("AAA", |pos| pos != "ZZZ", map, directions)
}

fn part2(input: &(Directions, Map)) -> usize {
    let (directions, map) = input;
    map.keys()
        .filter(|node| node.ends_with('A'))
        .map(|pos| count_steps(pos, |pos| !pos.ends_with('Z'), map, directions))
        .fold(1, |lcm, clen| lcm.lcm(&clen))
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
