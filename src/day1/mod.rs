use aoc_downloader::download_day;

const DAY: u32 = 1;

const WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

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
        0, //part1(&input),
        part2(&input)
    );
}

fn part1(input: &Vec<String>) -> u32 {
    let mut output = Vec::new();
    for line in input {
        output.push(line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>());
    }
    let mut sum = 0;
    for set in output{
        sum += set.first().unwrap() * 10 + set.last().unwrap();
    }
    sum
}

fn part2(input: &Vec<String>) -> u32 {
    let output = input.iter()
        .map(|l| {
            l.replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
            }
        )
        .map(|l| l.chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>()
        ).collect::<Vec<_>>();
    let mut sum = 0;
    for set in output{
        println!("{:?}", set);
        sum += set.first().unwrap() * 10 + set.last().unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(71502, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(208191, part2(&input));
    }
}
