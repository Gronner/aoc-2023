use std::collections::HashMap;

use aoc_downloader::download_day;

const DAY: u32 = 14;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Vec<Vec<char>>;

fn parse_input(input: Vec<String>) -> Input {
    input.iter()
        .map(|row| row.chars().collect())
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

fn move_rocks(map: &Input) -> Input {
    let mut after = map.clone();
    for column in 0..map[0].len() {
        let mut northest = 0;
        for row in 0..map.len() {
            match map[row][column] {
                'O' => {
                    after[row][column] = '.';
                    after[northest][column] = 'O';
                    northest += 1;
                },
                '#' => northest = row + 1,
                _ => (),
            }
        }
    }
    after
}

fn compute_load(input: &Input) -> usize {
    let y_size = input.len();
    input.iter().enumerate()
        .flat_map(|(y, row)| row.iter().map(move |pos| if *pos == 'O' { y_size - y} else { 0 }))
        .sum()
}

fn part1(input: &Input) -> usize {
    compute_load(&move_rocks(input))
}

fn rotate(input: &Input) -> Input {
    let mut rotated = vec![vec!['.'; input.len()]; input[0].len()];
    for column in 0..input[0].len() {
        for row in 0..input.len() {
            rotated[column][input[0].len() - 1 - row] = input[row][column];
        }
    }
    rotated
}

fn part2(input: &Input) -> usize {
    let mut input = input.clone();
    let mut memory = HashMap::new();
    let mut cycle = 0;
    loop {
        cycle += 1;
        for _ in 0..4 {
            input = move_rocks(&input);
            input = rotate(&input);
        }
        if memory.contains_key(&input) {
            break;
        }
        memory.insert(input.clone(), cycle);
    }
    let cycle_start = *memory.get(&input).unwrap();
    let cycle_length = cycle - cycle_start;
    let final_cycle = (1_000_000_000 - cycle) % cycle_length;
    for _ in 0..final_cycle {
        for _ in 0..4 {
            input = move_rocks(&input);
            input = rotate(&input);
        }
    }
    compute_load(&input)
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
