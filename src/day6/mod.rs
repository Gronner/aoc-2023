use std::iter::zip;

use aoc_downloader::download_day;

const DAY: u32 = 6;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

#[derive(Clone, Copy, Debug)]
struct Race {
    pub duration: i64,
    pub record: i64,
}

impl Race {
    fn discriminat(&self) -> i64 {
        // D = t^2 - 4 * -1 * -d
        self.duration.pow(2) - 4 * self.record
    }

    fn get_roots(&self) -> (Option<i64>, Option<i64>) {
        let discriminat = self.discriminat();
        if discriminat == 0 {
            // x = -t + sqrt(D)
            return (Some(-self.duration + discriminat.isqrt()), None);
        }
        // x = (-t + sqrt(D))/-2 & x = (-v - sqrt(D))/-2
        (Some(((self.duration as f64 - (discriminat as f64).sqrt())/2.).floor() as i64 + 1), Some(((self.duration as f64 + (discriminat as f64).sqrt())/2.).ceil() as i64 - 1))

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

fn part1(input: &[String]) -> i64 {
    let times = input[0]
        .split(' ')
        .filter_map(|split| split.parse().ok())
        .collect::<Vec<i64>>();
    let distances = input[1]
        .split(' ')
        .filter_map(|split| split.parse().ok())
        .collect::<Vec<i64>>();
    zip(times, distances)
        .map(|(duration, record)| Race { duration, record })
        .map(|race| race.get_roots())
        .inspect(|s| println!("{:?}", s))
        .map(|(start, end)| end.unwrap() - start.unwrap() + 1)
        .product()
}

fn part2(input: &[String]) -> i64 {
    let roots = Race {
        duration: input[0]
            .split_whitespace()
            .collect::<String>()
            .split(':')
            .last()
            .unwrap()
            .parse()
            .unwrap(),
        record: input[1]
            .split_whitespace()
            .collect::<String>()
            .split(':')
            .last()
            .unwrap()
            .parse()
            .unwrap(),
    }.get_roots();
    roots.1.unwrap() - roots.0.unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(114400, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(21039729, part2(&input));
    }
}
