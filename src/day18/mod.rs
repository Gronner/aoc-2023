use std::str::FromStr;

use aoc_downloader::download_day;

const DAY: u32 = 18;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Pos = (i128, i128);
type Input = Vec<Command>;

#[derive(Debug)]
struct Command {
    pub direction: Pos,
    pub color: String,
}

#[derive(Debug)]
struct Command2 {
    pub direction: Pos,
}

impl From<&Command> for Command2 {
    fn from(value: &Command) -> Self {
        let distance = i128::from_str_radix(&value.color[..5], 16).unwrap();
        let direction = match &value.color[5..] {
            "0" => (0, distance),
            "1" => (distance, 0),
            "2" => (0, -distance),
            "3" => (-distance, 0),
            e => panic!("Unexpected direction: {e}"),
        };

        Command2 { direction }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"(\w) (\d+) \(\#(\w+)\)");
        let captured = re.captures(s).unwrap();
        let dir = captured.get(1).unwrap().as_str();
        let distance: i128 = captured.get(2).unwrap().as_str().parse().unwrap();
        let color = captured.get(3).unwrap().as_str().to_string();

        let direction = match dir {
            "U" => (-distance, 0),
            "D" => (distance, 0),
            "L" => (0, -distance),
            "R" => (0, distance),
            e => panic!("Unexpected direction: {e}"),
        };

        Ok(Command { direction, color })
    }
}


fn parse_input(input: Vec<String>) -> Input {
    input.iter()
        .map(|line| Command::from_str(&line).unwrap())
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

fn determinate(lhs: &Pos, rhs: &Pos) -> i128 {
    lhs.0 * rhs.1 - lhs.1 * rhs.0
}

fn part1(input: &Input) -> i128 {
    let mut pos = (0, 0);
    let mut boundary = vec![pos];
    input.iter()
        .for_each(|com| {
            let new_pos = (pos.0 + com.direction.0, pos.1 + com.direction.1);
            while pos != new_pos {
                pos = (pos.0 + com.direction.0.signum(), pos.1 + com.direction.1.signum());
                boundary.push(pos);
            }
            pos = new_pos;
        });

    boundary
        .windows(2)
        .map(|win| determinate(&win[0], &win[1]))
        .sum::<i128>()
        .abs()
        / 2
        + boundary.len() as i128 / 2 + 1
}

fn part2(input: &Input) -> i128 {
    let mut pos = (0, 0);
    let mut boundary = vec![pos];
    input.iter()
        .map(|com| Command2::from(com))
        .for_each(|com| {
            let new_pos = (pos.0 + com.direction.0, pos.1 + com.direction.1);
            while pos != new_pos {
                pos = (pos.0 + com.direction.0.signum(), pos.1 + com.direction.1.signum());
                boundary.push(pos);
            }
            pos = new_pos;
        });

    boundary
        .windows(2)
        .map(|win| determinate(&win[0], &win[1]))
        .sum::<i128>()
        .abs()
        / 2
        + boundary.len() as i128 / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(38188, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(93325849869340, part2(&input));
    }
}
