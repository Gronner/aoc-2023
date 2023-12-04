use std::{str::FromStr, collections::{HashMap}};

use aoc_downloader::download_day;

const DAY: u32 = 4;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

#[derive(Debug)]
struct Game {
    pub id: usize,
    winners: Vec<u64>,
    numbers: Vec<u64>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let game_re = regex!(r"Card\s+(\d+):\s+((?:\d+\s+)*)\|\s+((?:\d+(?:\s+)?)*)");
        let matches = game_re.captures(s).unwrap_or_else(|| panic!("{}", s));
        let id = matches.get(1).unwrap().as_str().parse().unwrap();
        let winners = matches.get(2).unwrap().as_str().split(char::is_whitespace).filter(|s| !s.is_empty()).map(|n| n.parse().unwrap()).collect();
        let numbers = matches.get(3).unwrap().as_str().split(char::is_whitespace).filter(|s| !s.is_empty()).map(|n| n.parse().unwrap()).collect();
        Ok(Game { id, winners, numbers })
    }
}

impl Game {
    fn get_matches(&self) -> u64 {
        self.winners.iter()
            .map(|w| if self.numbers.iter().any(|n| n == w) { 1 } else { 0 })
            .sum::<u64>()
    }

    fn get_score(&self) -> u64 {
        let result = self.get_matches();
        if result != 0 {
            2_u64.pow((result - 1) as u32)
        } else {
            result
        }
    }

    fn is_winner(&self) -> bool {
        self.winners.iter()
            .any(|w| self.numbers.iter().any(|n| n == w))
    }
}

fn parse_input(input: Vec<String>) -> Vec<Game> {
    input.iter()
        .map(|line| Game::from_str(line).unwrap())
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

fn part1(input: &[Game]) -> u64 {
    input.iter()
        .map(|g| g.get_score())
        .sum()
}

fn count_winning_cards(card: usize, winner_cards: &HashMap<usize, u64>) -> u64 {
    if !winner_cards.contains_key(&card) {
        return 1;
    }
    let mut card_count = 1;
    for c in (card + 1)..=(card + *winner_cards.get(&card).unwrap() as usize) {
        card_count += count_winning_cards(c, winner_cards)
    }
    card_count
}

fn part2(input: &[Game]) -> u64 {
    let mut winning_cards = HashMap::new();
    input.iter()
        .filter(|card| card.is_winner())
        .for_each(|card| { winning_cards.insert(card.id, card.get_matches()); });
    winning_cards.keys()
        .map(|card| count_winning_cards(*card, &winning_cards))
        .sum::<u64>() + input.len() as u64 - winning_cards.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(23673, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(12263631, part2(&input));
    }
}
