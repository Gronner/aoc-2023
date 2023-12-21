use num::Integer;
use pathfinding::prelude::yen;
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use aoc_downloader::download_day;

const DAY: u32 = 20;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = HashMap<String, Box<dyn Pulse>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Level {
    High,
    Low,
}

impl Level {
    fn toggle(&mut self) {
        if *self == Self::High {
            *self = Self::Low;
        } else {
            *self = Self::High;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Type {
    Conjunction,
    FlipFlop,
    Broadcaster,
}

trait Pulse {
    fn pulse(&mut self, origin: &str, pulse: Level) -> Option<(Level, Vec<String>)>;

    fn mod_type(&self) -> Type;

    fn next(&self) -> Vec<String>;

    fn name(&self) -> String;

    fn add_inputs(&mut self, inputs: Vec<String>);

    fn get_state(&self) -> Level;
}

#[derive(Clone, Debug)]
struct FlipFlop {
    name: String,
    next: Vec<String>,
    state: Level,
}

impl FromStr for FlipFlop {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"(\w+) -> (.*)");
        let captured = re.captures(s).unwrap();
        let name = captured.get(1).unwrap().as_str().to_string();
        let next = captured
            .get(2)
            .unwrap()
            .as_str()
            .split(',')
            .map(|split| split.trim().to_string())
            .collect::<Vec<String>>();

        Ok(Self {
            name,
            next,
            state: Level::Low,
        })
    }
}

impl Pulse for FlipFlop {
    fn pulse(&mut self, _: &str, pulse: Level) -> Option<(Level, Vec<String>)> {
        if pulse == Level::Low {
            self.state.toggle();
            Some((self.state, self.next.clone()))
        } else {
            None
        }
    }

    fn mod_type(&self) -> Type {
        Type::FlipFlop
    }

    fn next(&self) -> Vec<String> {
        self.next.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn add_inputs(&mut self, _: Vec<String>) {
        unreachable!()
    }

    fn get_state(&self) -> Level {
        self.state
    }
}

#[derive(Clone, Debug)]
struct Conjunction {
    name: String,
    next: Vec<String>,
    state: HashMap<String, Level>,
}

impl FromStr for Conjunction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"(\w+) -> (.*)");
        let captured = re.captures(s).unwrap();
        let name = captured.get(1).unwrap().as_str().to_string();
        let next = captured
            .get(2)
            .unwrap()
            .as_str()
            .split(',')
            .map(|split| split.trim().to_string())
            .collect::<Vec<String>>();

        Ok(Self {
            name,
            next,
            state: HashMap::new(),
        })
    }
}

impl Pulse for Conjunction {
    fn pulse(&mut self, origin: &str, pulse: Level) -> Option<(Level, Vec<String>)> {
        *self.state.get_mut(origin).unwrap() = pulse;
        if self.state.values().all(|level| *level == Level::High) {
            Some((Level::Low, self.next.clone()))
        } else {
            Some((Level::High, self.next.clone()))
        }
    }

    fn mod_type(&self) -> Type {
        Type::Conjunction
    }

    fn next(&self) -> Vec<String> {
        self.next.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn add_inputs(&mut self, inputs: Vec<String>) {
        inputs.iter().for_each(|input| {
            self.state.insert(input.to_string(), Level::Low);
        });
    }

    fn get_state(&self) -> Level {
        if self.state.values().all(|level| *level == Level::Low) {
            Level::Low
        } else {
            Level::High
        }
    }
}

#[derive(Clone, Debug)]
struct Broadcaster {
    name: String,
    next: Vec<String>,
}

impl FromStr for Broadcaster {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"(\w+) -> (.*)");
        let captured = re.captures(s).unwrap();
        let name = captured.get(1).unwrap().as_str().to_string();
        let next = captured
            .get(2)
            .unwrap()
            .as_str()
            .split(',')
            .map(|split| split.trim().to_string())
            .collect::<Vec<String>>();

        Ok(Self { name, next })
    }
}

impl Pulse for Broadcaster {
    fn pulse(&mut self, _: &str, pulse: Level) -> Option<(Level, Vec<String>)> {
        Some((pulse, self.next.clone()))
    }

    fn mod_type(&self) -> Type {
        Type::Broadcaster
    }

    fn next(&self) -> Vec<String> {
        self.next.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn add_inputs(&mut self, _: Vec<String>) {
        unreachable!()
    }

    fn get_state(&self) -> Level {
        Level::Low
    }
}

fn create_module(line: &str) -> (String, Box<dyn Pulse>) {
    if let Some(stripped) = line.strip_prefix('%') {
        let module = FlipFlop::from_str(stripped).unwrap();
        (module.name.clone(), Box::new(module))
    } else if let Some(stripped) = line.strip_prefix('&') {
        let module = Conjunction::from_str(stripped).unwrap();
        (module.name.clone(), Box::new(module))
    } else {
        let module = Broadcaster::from_str(line).unwrap();
        (module.name.clone(), Box::new(module))
    }
}

fn parse_input(input: Vec<String>) -> Input {
    let mut modules = input
        .iter()
        .map(|line| create_module(line))
        .collect::<HashMap<String, Box<dyn Pulse>>>();

    let mut input_map = modules
        .iter()
        .filter(|(_, module)| module.mod_type() == Type::Conjunction)
        .map(|(name, _)| (name.clone(), vec![]))
        .collect::<HashMap<_, _>>();

    modules.iter().for_each(|(name, module)| {
        module.next().iter().for_each(|n| {
            if let Some(inp) = input_map.get_mut(n) {
                inp.push(name.clone());
            }
        })
    });

    input_map.iter().for_each(|(module, inputs)| {
        if let Some(m) = modules.get_mut(module.as_str()) {
            m.add_inputs(inputs.clone());
        }
    });

    modules
}

pub fn run_day() {
    let input = get_input();
    let mut input1 = parse_input(input);
    let input = get_input();
    let mut input2 = parse_input(input);
    println!(
        "Running day {}:\n\tPart1 {}\n\tPart2 {}",
        DAY,
        part1(&mut input1),
        part2(&mut input2)
    );
}

fn part1(input: &mut Input) -> usize {
    let modules: &mut Input = input;
    let mut lows = 0;
    let mut highs = 0;
    for _ in 0..1000 {
        let mut stack = VecDeque::from([(
            String::from("button"),
            Level::Low,
            String::from("broadcaster"),
        )]);
        while let Some((origin, level, next)) = stack.pop_back() {
            if level == Level::Low {
                lows += 1;
            } else {
                highs += 1;
            }
            if next == "rx" {
                continue;
            }
            let module = modules.get_mut(&next).unwrap();
            let name = module.name();
            if let Some((new_level, nexts)) = module.pulse(&origin, level) {
                for next in nexts.iter() {
                    stack.push_front((name.clone(), new_level, next.to_string()));
                }
            } else {
                continue;
            }
        }
    }
    lows * highs
}

fn part2(input: &mut Input) -> u128 {
    let modules: &mut Input = input;
    let mut paths = Vec::new();
    // NANDs before the final NANDs' final NAND
    let goals = vec!["hz", "xc", "gh", "cn"];
    for goal in goals {
        paths.push(
            yen(
                &String::from("broadcaster"),
                |node| {
                    if let Some(module) = modules.get(node.as_str()) {
                        module
                            .next()
                            .iter()
                            .map(|s| (s.to_string(), 1))
                            .collect::<Vec<(String, usize)>>()
                    } else {
                        vec![]
                    }
                },
                |node| node == goal,
                100,
            )
            .iter()
            .last()
            .map(|(path, _)| path)
            .unwrap()
            .clone(),
        );
    }
    paths
        .iter()
        .map(|path| {
            u128::from_str_radix(
                &path
                    .iter()
                    .skip(1) // Don't care about broadcaster
                    // Compute Mod Counter Value
                    .filter(|node| {
                        modules.get::<String>(node).unwrap().mod_type() != Type::Conjunction
                    })
                    .map(|node| {
                        if modules
                            .get(node)
                            .unwrap()
                            .next()
                            .iter()
                            .any(|next| modules.get(next).unwrap().mod_type() == Type::Conjunction)
                        {
                            '1'
                        } else {
                            '0'
                        }
                    })
                    .rev()
                    .collect::<String>(),
                2,
            )
            .unwrap()
        })
        .fold(1, |lcm, clen| lcm.lcm(&clen))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        let sample = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"
        .split('\n')
        .map(|split| split.to_string())
        .filter(|split| !split.is_empty())
        .collect::<Vec<String>>();
        let mut input = parse_input(sample);
        assert_eq!(32000000, part1(&mut input));
    }

    #[test]
    fn part1_sample2() {
        let sample = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> rx 
"
        .split('\n')
        .map(|split| split.to_string())
        .filter(|split| !split.is_empty())
        .collect::<Vec<String>>();
        let mut input = parse_input(sample);
        assert_eq!(11687500, part1(&mut input));
    }

    #[test]
    fn part1_output() {
        let mut input = parse_input(get_input());
        assert_eq!(896998430, part1(&mut input));
    }

    #[test]
    fn part2_output() {
        let mut input = parse_input(get_input());
        assert_eq!(236095992539963, part2(&mut input));
    }
}
