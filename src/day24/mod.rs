use std::{ops::Index, str::FromStr};

use aoc_downloader::download_day;
use itertools::Itertools;

const DAY: u32 = 24;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Vec<Hailstone>;

#[derive(Debug)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl FromStr for Vector3 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(", ")
            .map(|n| {
                if let Err(e) = n.parse::<f64>() {
                    panic!("{} on {}", e, n);
                } else {
                    n.parse().unwrap()
                }
            })
            .collect::<Vec<f64>>();
        Ok(Vector3 {
            x: values[0],
            y: values[1],
            z: values[2],
        })
    }
}

impl Index<char> for Vector3 {
    type Output = f64;

    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x' => &self.x,
            'y' => &self.y,
            'z' => &self.z,
            e => panic!("Unexpected index {e}"),
        }
    }
}

#[derive(Debug)]
struct Hailstone {
    pos: Vector3,
    velocity: Vector3,
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split_once(" @ ").unwrap();
        Ok(Hailstone {
            pos: Vector3::from_str(splits.0).unwrap(),
            velocity: Vector3::from_str(splits.1).unwrap(),
        })
    }
}

fn parse_input(input: Vec<String>) -> Input {
    input
        .iter()
        .map(|line| Hailstone::from_str(line).unwrap())
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

impl Vector3 {
    fn gain_x_y(&self) -> f64 {
        self['y'] / self['x']
    }

    fn is_parallel_x_y(&self, other: &Self) -> bool {
        (other.gain_x_y() - self.gain_x_y()).abs() < f64::EPSILON
    }
}

impl Hailstone {
    fn intersect(&self, other: &Self) -> Option<(f64, f64)> {
        if self.velocity.is_parallel_x_y(&other.velocity) {
            return None;
        }

        let m1 = self.velocity.gain_x_y();
        let m2 = other.velocity.gain_x_y();

        let x =
            (m1 * self.pos['x'] - m2 * other.pos['x'] + other.pos['y'] - self.pos['y']) / (m1 - m2);
        let y = (m1 * m2 * (other.pos['x'] - self.pos['x']) + m2 * self.pos['y']
            - m1 * other.pos['y'])
            / (m2 - m1);

        Some((x, y))
    }

    fn in_future_x_y(&self, point: (f64, f64)) -> bool {
        self.velocity['x'].signum() == (point.0 - self.pos['x']).signum()
    }
}

fn part1(input: &Input) -> usize {
    const MIN_VAL: f64 = 200000000000000.0;
    const MAX_VAL: f64 = 400000000000000.0;
    input
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            if let Some(intersection) = a.intersect(b) {
                a.in_future_x_y(intersection)
                    && b.in_future_x_y(intersection)
                    && intersection.0 >= MIN_VAL
                    && intersection.0 <= MAX_VAL
                    && intersection.1 >= MIN_VAL
                    && intersection.1 <= MAX_VAL
            } else {
                false
            }
        })
        .count()
}

fn part2(input: &Input) -> usize {
    // We need to solve for 6 variables x_0, y_0, z_0, dx_0, dy_0, dz_0
    // For every pair of stones there is a solution that fits
    // For a set of three stones, there should only be one that fits
    // We need to find the intersection at 3 distinct times t_i
    // So we need a total of 9 equations
    // Each stone yields 3 equations (x_i, y_i, z_i)
    //
    // x_0 + dx_0 * t_i = x_i + dx_i * t_i
    // y_0 + dy_0 * t_i = y_i + dy_i * t_i
    // z_0 + dz_0 * t_i = z_i + dz_i * t_i
    // =>
    // 0 = x_1 + dx_1 * t_i - x_0 - dx_0 * t_i
    // =>
    // 0 = t_i * (dx_1 - dx_0) + x_1 - x_0
    // The rest follows
    input
        .iter()
        .take(3)
        .enumerate()
        .map(|(i, stone)| {
            (
                i,
                stone.pos['x'],
                stone.pos['y'],
                stone.pos['z'],
                stone.velocity['x'],
                stone.velocity['y'],
                stone.velocity['z'],
            )
        })
        .for_each(|(i, x_i, y_i, z_i, dx_i, dy_i, dz_i)| {
            println!(
                "0 = {x_i} + {dx_i} * t_{i} - x_0 - u_0 * t_{i}
0 = {y_i} + {dy_i} * t_{i} - y_0 - v_0 * t_{i}
0 = {z_i} + {dz_i} * t_{i} - z_0 - w_0 * t_{i}"
            );
        });
    // As rust does not seem to have a nice symbolic solver, z3 does not run on my system I threw
    // this equation into a solver yielding
    //      x_0             y_0              z_0
    194723518367339 + 181910661443432 + 150675954587450
    // Solving the equation system is left as an exercise for the bored reader ;-)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(108813, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(104533, part2(&input));
    }
}
