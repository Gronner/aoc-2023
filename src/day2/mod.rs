use std::str::FromStr;

use aoc_downloader::download_day;

const DAY: u32 = 2;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game_re = regex!(r"Game (\d+)");
        let id = game_re.captures(s).unwrap();
        let id = id.get(1).unwrap().as_str().parse().unwrap();
        let rounds = s
            .split(": ")
            .last()
            .unwrap()
            .split(';')
            .map(|r| Round::from_str(r).unwrap())
            .collect();
        Ok(Game { id, rounds })
    }
}

impl Game {
    fn is_valid(&self, red: u32, green: u32, blue: u32) -> bool {
        !self
            .rounds
            .iter()
            .any(|r| r.red > red || r.green > green || r.blue > blue)
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_minimal_round_set(&self) -> Round {
        let red = self.rounds.iter().map(|r| r.red).max().unwrap();

        let green = self.rounds.iter().map(|r| r.green).max().unwrap();

        let blue = self.rounds.iter().map(|r| r.blue).max().unwrap();

        Round { red, blue, green }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

fn extract_count(regex: &regex::Regex, input: &str) -> u32 {
    if let Some(redc) = regex.captures(input) {
        redc.get(1).unwrap().as_str().parse().unwrap()
    } else {
        0
    }
}

impl FromStr for Round {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let red_re = regex!(r"(\d+) red");
        let green_re = regex!(r"(\d+) green");
        let blue_re = regex!(r"(\d+) blue");

        let red = extract_count(red_re, s);
        let green = extract_count(green_re, s);
        let blue = extract_count(blue_re, s);

        Ok(Round { red, blue, green })
    }
}

fn parse_input(input: Vec<String>) -> Vec<Game> {
    input.iter().map(|l| Game::from_str(l).unwrap()).collect()
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

fn part1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|g| g.is_valid(12, 13, 14))
        .fold(0, |acc, g| acc + g.get_id())
}

fn part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|g| g.get_minimal_round_set())
        .fold(0, |acc, r| acc + r.red * r.green * r.blue)
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
