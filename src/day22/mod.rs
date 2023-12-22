use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use aoc_downloader::download_day;

const DAY: u32 = 22;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Vec<Brick>;
//               x  ,   y  ,   z
type Corner = (usize, usize, usize);
type Pos = (usize, usize);

#[derive(Clone, Copy, Debug)]
struct Brick {
    // lower
    pub corner_a: Corner,
    // upper
    pub corner_b: Corner,
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split('~').collect::<Vec<_>>();
        let corner_a: Corner = splits[0]
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let corner_b: Corner = splits[1]
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        if corner_a.2 <= corner_b.2 {
            Ok(Self { corner_a, corner_b })
        } else {
            Ok(Self { corner_b, corner_a })
        }
    }
}

fn parse_input(input: Vec<String>) -> Input {
    input
        .iter()
        .map(|line| Brick::from_str(line).unwrap())
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

impl Brick {
    fn get_upper_z(&self) -> usize {
        self.corner_b.2
    }

    fn get_lower_z(&self) -> usize {
        self.corner_a.2
    }

    fn get_surface(&self) -> Vec<Pos> {
        (self.corner_a.0..=self.corner_b.0)
            .cartesian_product(self.corner_a.1..=self.corner_b.1)
            .collect()
    }
}

fn fall(input: &Input) -> Input {
    let mut floor = HashMap::new();
    let mut input = input.clone();
    input.sort_by_key(|a| a.get_lower_z());
    for brick in input.iter_mut() {
        let new_height = if let Some(h) = brick
            .get_surface()
            .iter()
            .filter_map(|pos| floor.get(pos))
            .max()
        {
            h + 1
        } else {
            1
        };

        let height = brick.corner_b.2 - brick.corner_a.2;
        brick.corner_a.2 = new_height;
        brick.corner_b.2 = brick.corner_a.2 + height;
        brick.get_surface().iter().for_each(|pos| {
            floor
                .entry(*pos)
                .and_modify(|z| *z = brick.corner_b.2)
                .or_insert(brick.corner_b.2);
        });
    }
    input
}

fn get_stacks(input: &Input) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) {
    let mut below_of: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut above_of: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, brick) in input.iter().enumerate() {
        for (j, other_brick) in input.iter().enumerate() {
            if i == j {
                continue;
            }
            if (brick.get_upper_z() + 1) == other_brick.get_lower_z()
                && !HashSet::<Pos>::from_iter(brick.get_surface().iter().cloned()).is_disjoint(
                    &HashSet::from_iter(other_brick.get_surface().iter().cloned()),
                )
            {
                below_of
                    .entry(i)
                    .and_modify(|below| below.push(j))
                    .or_insert(vec![j]);
                above_of
                    .entry(j)
                    .and_modify(|above| above.push(i))
                    .or_insert(vec![i]);
            } else {
                below_of.entry(i).or_default();
                above_of.entry(j).or_default();
            }
        }
    }
    (below_of, above_of)
}

fn part1(input: &Input) -> usize {
    let input = fall(input);
    let (_, above_of) = get_stacks(&input);

    let mut destroyable = vec![true; above_of.len()];

    for below in above_of.values() {
        if below.len() == 1 {
            destroyable[below[0]] = false;
        }
    }

    destroyable.iter().filter(|&s| *s).count()
}

fn part2(input: &Input) -> usize {
    let input = fall(input);

    let (below_of, above_of) = get_stacks(&input);

    let mut safe = vec![true; above_of.len()];

    for below in above_of.values() {
        if below.len() == 1 {
            safe[below[0]] = false;
        }
    }

    let mut result = 0;
    let mut queue = VecDeque::new();

    for block in below_of.iter().filter(|(&b, _)| !safe[b]).map(|(b, _)| b) {
        queue.push_back(*block);
        let mut removed = HashSet::new();
        removed.insert(*block);

        while let Some(current) = queue.pop_front() {
            for &next in below_of.get(&current).unwrap() {
                if !removed.contains(&next)
                    && above_of
                        .get(&next)
                        .unwrap()
                        .iter()
                        .all(|&i| removed.contains(&i))
                {
                    result += 1;
                    removed.insert(next);
                    queue.push_back(next);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(395, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(64714, part2(&input));
    }
}
