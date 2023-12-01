use aoc_downloader::download_day;

const DAY: u32 = 1;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
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

fn calculate(input: &[String], replace: bool) -> u32 {
    let output = input.iter()
        .map(|l| { if replace {
            l.replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
            } else {
                l.to_string()
            }}
        )
        .map(|l| l.chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>()
        ).collect::<Vec<_>>();
    output.iter()
        .map(|s| s.first().unwrap() * 10 + s.last().unwrap())
        .sum()
}

fn part1(input: &[String]) -> u32 {
    calculate(input, false)
}

fn part2(input: &[String]) -> u32 {
    calculate(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(54390, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(54277, part2(&input));
    }
}
