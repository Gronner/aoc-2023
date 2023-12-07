use std::str::FromStr;
use std::collections::HashMap;
use std::iter::zip;

use aoc_downloader::download_day;

const DAY: u32 = 7;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(u64),
}

impl Card {
    fn to_value(&self) -> u64 {
        match self {
            Self::Ace => 14,
            Self::King => 13,
            Self::Queen => 12,
            Self::Jack => 11,
            Self::Number(n) => *n,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Number(10),
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

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.partial_cmp(self).unwrap()
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_value().partial_cmp(&other.to_value())
    }
}

#[derive(Clone, Debug, Eq)]
enum Type {
    FiveOfAKind(Card),
    FourOfAKind(Card),
    FullHouse(Card, Card),
    ThreeOfAKind(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    HighCard(Card),
}

impl Type {
    fn to_value(&self) -> u64 {
        match self {
            Self::FiveOfAKind(_) => 7,
            Self::FourOfAKind(_) => 6,
            Self::FullHouse(_, _) => 5,
            Self::ThreeOfAKind(_) => 4,
            Self::TwoPair(_, _) => 3,
            Self::OnePair(_) => 2,
            Self::HighCard(_) => 1,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.partial_cmp(self).unwrap()
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_value().partial_cmp(&other.to_value())
    }
}

impl Type {
    fn from_cards(cards: &[Card]) -> Self {
        let mut card_frequency = HashMap::new();
        cards.iter()
            .for_each(|c| { card_frequency.entry(c).and_modify(|v| {*v +=1;}).or_insert(1); });
        let max_card = card_frequency.iter().max_by_key(|(_, v)| *v).map(|(&k, &v)| (*k, v)).unwrap();
        card_frequency.remove(&max_card.0);
        match max_card.1 {
            1 => Type::HighCard(max_card.0),
            2 => {
                let max_card_2 = card_frequency.iter().max_by_key(|(_, v)| *v).map(|(&k, &v)| (*k, v)).unwrap();
                if max_card_2.1 == 2 {
                    Type::TwoPair(max_card.0, max_card_2.0)
                } else {
                    Type::OnePair(max_card.0)
                }
            },
            3 => {
                let max_card_2 = card_frequency.iter().max_by_key(|(_, v)| *v).map(|(&k, &v)| (*k, v)).unwrap();
                if max_card_2.1 == 2 {
                    Type::FullHouse(max_card.0, max_card_2.0)
                } else {
                    Type::ThreeOfAKind(max_card.0)
                }
            },
            4 => Type::FourOfAKind(max_card.0),
            5 => Type::FiveOfAKind(max_card.0),
            n => panic!("To many cards: {}", n),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Ord)]
struct Hand {
    hand: Type,
    cards: Vec<Card>,
    pub bid: u64,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand.cmp(&other.hand) {
            std::cmp::Ordering::Equal => Some(zip(&self.cards, &other.cards).map(|(s, o)| s.cmp(&o)).filter(|&result| result != std::cmp::Ordering::Equal).collect::<Vec<_>>()[0]),
            ord => Some(ord),
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        let cards: Vec<Card> = split[0].chars().map(|c| Card::from(c)).collect();
        Ok(Hand {
            hand: Type::from_cards(&cards),
            cards,
            bid: split[1].parse().unwrap(),
        })
    }
}

fn parse_input(input: Vec<String>) -> Vec<Hand> {
    input.iter()
        .map(|s| Hand::from_str(s).unwrap())
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

fn part1(input: &Vec<Hand>) -> u64 {
    let mut input = input.clone();
    input.sort();
    input.iter()
        .rev()
        .enumerate()
        //.inspect(|(rank, hand)| println!("{:?}: {:?}", rank, hand))
        .map(|(rank, hand)| (rank + 1) as u64 * hand.bid)
        .sum()
}

fn part2(input: &[Hand]) -> u64 {
    0
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
