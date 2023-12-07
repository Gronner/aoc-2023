use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::str::FromStr;

use aoc_downloader::download_day;

const DAY: u32 = 7;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Number(u64),
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Number(10),
            'X' => Self::Joker,
            e => {
                if e.is_ascii_digit() {
                    Self::Number(e.to_digit(10).unwrap() as u64)
                } else {
                    panic!("Unkown card: {}", e);
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

impl Type {
    fn from_cards(cards: &[Card]) -> Self {
        let mut card_frequency = BTreeMap::new();
        cards.iter().for_each(|c| {
            card_frequency
                .entry(c)
                .and_modify(|v| {
                    *v += 1;
                })
                .or_insert(1);
        });
        let jokers = if let Some(j) = card_frequency.get(&Card::Joker) {
            *j
        } else {
            0
        };
        card_frequency.remove(&Card::Joker);
        let max_card = if jokers != 5 {
            card_frequency
                .iter()
                .max_by_key(|(_, v)| *v)
                .map(|(&k, &v)| (*k, v))
                .unwrap()
        } else {
            (Card::Joker, 0)
        };
        card_frequency.remove(&max_card.0);
        match max_card.1 + jokers {
            1 => Type::HighCard(cards.to_vec()),
            2 => {
                let max_card_2 = card_frequency
                    .iter()
                    .max_by_key(|(_, v)| *v)
                    .map(|(&k, &v)| (*k, v))
                    .unwrap();
                if max_card_2.1 == 2 {
                    Type::TwoPair(cards.to_vec())
                } else {
                    Type::OnePair(cards.to_vec())
                }
            }
            3 => {
                let max_card_2 = card_frequency
                    .iter()
                    .max_by_key(|(_, v)| *v)
                    .map(|(&k, &v)| (*k, v))
                    .unwrap();
                if max_card_2.1 == 2 {
                    Type::FullHouse(cards.to_vec())
                } else {
                    Type::ThreeOfAKind(cards.to_vec())
                }
            }
            4 => Type::FourOfAKind(cards.to_vec()),
            5 => Type::FiveOfAKind(cards.to_vec()),
            n => panic!("To many cards: {}", n),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Hand {
    hand: Type,
    pub bid: u64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        let cards: Vec<Card> = split[0].chars().map(Card::from).collect();
        Ok(Hand {
            hand: Type::from_cards(&cards),
            bid: split[1].parse().unwrap(),
        })
    }
}

fn parse_input(input: Vec<String>) -> Vec<String> {
    input
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

fn play_cards(input: &[Hand]) -> u64 {
    let mut input = input.to_vec();
    input.sort_by(|a, b| b.cmp(a));
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as u64 * hand.bid)
        .sum()
}

fn part1(input: &[String]) -> u64 {
    play_cards(
        &input
            .iter()
            .map(|s| Hand::from_str(s).unwrap())
            .collect::<Vec<_>>(),
    )
}

fn part2(input: &[String]) -> u64 {
    play_cards(
        &input
            .iter()
            .map(|s| s.replace('J', "X"))
            .map(|s| Hand::from_str(&s).unwrap())
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(246163188, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(245794069, part2(&input));
    }
}
