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
    pub duration: u64,
    pub record: u64,
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

fn run_race(race: &Race) -> u64 {
    let mut ways_to_win = 0;
    let mut speed = race.duration - 1;
    while (race.duration - speed) != 0 {
        let distance = (race.duration - speed) * speed;
        if distance > race.record {
            ways_to_win += 1;
        } 
        if let Some(s) = speed.checked_sub(1) {
            speed = s;
        } else {
            break;
        }
    }
    ways_to_win
}

fn part1(input: &[String]) -> u64 {
    let times = input[0].split(' ').filter_map(|split| split.parse().ok()).collect::<Vec<u64>>();
    let distances  = input[1].split(' ').filter_map(|split| split.parse().ok()).collect::<Vec<u64>>();
    let races = zip(times, distances).map(|(duration, record)| Race { duration, record}).collect::<Vec<Race>>();

    let mut result = 1;
    for race in races {
        result *= run_race(&race);
    }
    result
}

fn part2(input: &[String]) -> u64 {
    let race = Race {
        duration: input[0].split_whitespace().collect::<String>().split(':').last().unwrap().parse().unwrap(),
        record: input[1].split_whitespace().collect::<String>().split(':').last().unwrap().parse().unwrap(),
    };
    run_race(&race)
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
