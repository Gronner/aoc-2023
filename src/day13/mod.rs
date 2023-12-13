use std::iter::zip;

use aoc_downloader::download_day;

const DAY: u32 = 13;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Vec<Vec<Vec<char>>>;

fn parse_input(input: Vec<String>) -> Input {
    let mut fields = Vec::new();
    let mut field = Vec::new();
    for line in input {
        if line.is_empty() {
            fields.push(field.clone());
            field.clear();
            continue;
        }
        field.push(line.chars().collect());
    }
    fields.push(field);
    fields
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

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn line_to_number(line: &[char]) -> usize {
    usize::from_str_radix(
        &line
            .iter()
            .map(|c| if *c == '.' { '0' } else { '1' })
            .collect::<String>(),
        2,
    )
    .unwrap()
}

fn find_symmetry(grid: &[Vec<char>]) -> Option<usize> {
    let encoded = grid
        .iter()
        .map(|line| line_to_number(line))
        .collect::<Vec<usize>>();

    for i in 1..encoded.len() {
        let (left, right) = encoded.split_at(i);
        let left = left.iter().rev().copied().collect::<Vec<usize>>();

        let size = left.len().min(right.len());
        if left[..size] == right[..size] {
            return Some(i);
        }
    }
    None
}

fn get_symmetry_line(grid: &Vec<Vec<char>>) -> usize {
    if let Some(normal) = find_symmetry(grid) {
        return normal * 100;
    }
    if let Some(transposed) = find_symmetry(&transpose(grid.to_vec())) {
        return transposed;
    }
    panic!("No symmetry found");
}

fn part1(input: &Input) -> usize {
    input.iter().map(get_symmetry_line).sum()
}

fn get_smudge(grid: &Vec<Vec<char>>) -> Option<usize> {
    let encoded = grid
        .iter()
        .map(|line| line_to_number(line))
        .collect::<Vec<usize>>();

    for i in 1..encoded.len() {
        let (left, right) = encoded.split_at(i);

        let difference = zip(left.iter().rev(), right)
            .filter(|(l, r)| l != r)
            .map(|(l, r)| l ^ r)
            .collect::<Vec<usize>>();
        // As there is only one defect in the grid there must a) be only on difference
        // and b) that difference must be a power of two (one bit)
        if difference.len() == 1 && (difference[0] & (difference[0] - 1)) == 0 {
            return Some(i);
        }
    }
    None
}

fn get_fixed_symmetry_line(grid: &Vec<Vec<char>>) -> usize {
    if let Some(normal) = get_smudge(grid) {
        return normal * 100;
    }
    if let Some(transposed) = get_smudge(&transpose(grid.to_vec())) {
        return transposed;
    }
    panic!("No symmetry found");
}

fn part2(input: &Input) -> usize {
    input.iter().map(get_fixed_symmetry_line).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(36041, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(35915, part2(&input));
    }
}
