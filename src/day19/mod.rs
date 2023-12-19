use std::{collections::HashMap, str::FromStr};

use aoc_downloader::download_day;

const DAY: u32 = 19;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Workflows = HashMap<String, Vec<Check>>;
type Input = (Workflows, Vec<Part>);

#[derive(Debug)]
enum Check {
    Accepted,
    Rejected,
    Next(String),
    GT(String, usize, String),
    LT(String, usize, String),
}

fn parse_workflow(s: &str) -> (String, Vec<Check>) {
    let re = regex!(r"(\w+)\{(.*)\}");
    let captured = re.captures(s).unwrap();
    let name = captured.get(1).unwrap().as_str().to_string();

    let checks = captured.get(2).unwrap().as_str().to_string();

    let checks = checks
        .split(',')
        .map(|split| {
            if split == "A" {
                return Check::Accepted;
            }
            if split == "R" {
                return Check::Rejected;
            }
            if !split.contains(':') {
                return Check::Next(split.to_string());
            }

            // a<2006:qkq
            let (chk, target) = split.split_once(':').unwrap();

            if chk.contains('>') {
                let (variable, number) = chk.split_once('>').unwrap();
                Check::GT(
                    variable.to_string(),
                    number.parse().unwrap(),
                    target.to_string(),
                )
            } else if chk.contains('<') {
                let (variable, number) = chk.split_once('<').unwrap();
                Check::LT(
                    variable.to_string(),
                    number.parse().unwrap(),
                    target.to_string(),
                )
            } else {
                unreachable!("Unexpected check: {split}");
            }
        })
        .collect::<Vec<_>>();

    (name, checks)
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}");
        let captured = re.captures(s).unwrap();
        Ok(Part {
            x: captured.get(1).unwrap().as_str().parse().unwrap(),
            m: captured.get(2).unwrap().as_str().parse().unwrap(),
            a: captured.get(3).unwrap().as_str().parse().unwrap(),
            s: captured.get(4).unwrap().as_str().parse().unwrap(),
        })
    }
}

impl std::ops::Index<&str> for Part {
    type Output = usize;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "x" => &self.x,
            "m" => &self.m,
            "a" => &self.a,
            "s" => &self.s,
            e => panic!("Unexpected Index: {e}"),
        }
    }
}

impl Part {
    fn apply_workflow(&self, workflows: &Workflows) -> usize {
        let mut workflow = "in";

        loop {
            let checks = workflows
                .get(workflow)
                .expect("Non-transition workflow {workflow}");
            for check in checks {
                match check {
                    Check::Accepted => return self.compute_worth(),
                    Check::Rejected => return 0,
                    Check::Next(new_wf) => {
                        workflow = &new_wf;
                        break;
                    }
                    Check::GT(variable, number, next) => {
                        if self[variable] > *number {
                            workflow = &next;
                            break;
                        }
                    }
                    Check::LT(variable, number, next) => {
                        if self[variable] < *number {
                            workflow = &next;
                            break;
                        }
                    }
                }
            }
        }
    }

    fn compute_worth(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn parse_input(input: Vec<String>) -> Input {
    let mut workflows = input
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| parse_workflow(line))
        .collect::<HashMap<_, _>>();
    workflows.insert("A".to_string(), vec![Check::Accepted]);
    workflows.insert("R".to_string(), vec![Check::Rejected]);

    let parts = input
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| Part::from_str(line).unwrap())
        .collect::<Vec<_>>();

    (workflows, parts)
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

fn part1(input: &Input) -> usize {
    let (workflows, parts) = input;
    parts
        .iter()
        .map(|part| part.apply_workflow(workflows))
        .sum()
}

fn part2(input: &Input) -> usize {
    let (workflows, _) = input;
    let mut stack = vec![("in", 0, (1, 4000), (1, 4000), (1, 4000), (1, 4000))];
    let mut combinations = 0;
    while let Some((workflow, stage, x, m, a, s)) = stack.pop() {
        if workflow == "R" {
            continue;
        }
        if x.0 > x.1 || m.0 > m.1 || a.0 > a.1 || s.0 > s.1 {
            // No bounds
            continue;
        }
        if workflow == "A" {
            combinations += (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1);
            continue;
        }
        let check = &workflows.get(workflow).unwrap()[stage];
        match check {
            Check::Accepted => {
                combinations +=
                    (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1);
                continue;
            }
            Check::Rejected => {
                continue;
            }
            Check::Next(new_wf) => {
                stack.push((&new_wf, 0, x, m, a, s));
            }
            Check::GT(variable, number, next) => match variable.as_ref() {
                "x" => {
                    stack.push((&next, 0, (number + 1, x.1), m, a, s));
                    stack.push((workflow, stage + 1, (x.0, *number), m, a, s));
                }
                "m" => {
                    stack.push((&next, 0, x, (number + 1, m.1), a, s));
                    stack.push((workflow, stage + 1, x, (m.0, *number), a, s));
                }
                "a" => {
                    stack.push((&next, 0, x, m, (number + 1, a.1), s));
                    stack.push((workflow, stage + 1, x, m, (a.0, *number), s));
                }
                "s" => {
                    stack.push((&next, 0, x, m, a, (number + 1, s.1)));
                    stack.push((workflow, stage + 1, x, m, a, (s.0, *number)));
                }
                e => panic!("Unexpected variable: {e}"),
            },
            Check::LT(variable, number, next) => match variable.as_ref() {
                "x" => {
                    stack.push((&next, 0, (x.0, number - 1), m, a, s));
                    stack.push((workflow, stage + 1, (*number, x.1), m, a, s));
                }
                "m" => {
                    stack.push((&next, 0, x, (m.0, number - 1), a, s));
                    stack.push((workflow, stage + 1, x, (*number, m.1), a, s));
                }
                "a" => {
                    stack.push((&next, 0, x, m, (a.0, number - 1), s));
                    stack.push((workflow, stage + 1, x, m, (*number, a.1), s));
                }
                "s" => {
                    stack.push((&next, 0, x, m, a, (s.0, number - 1)));
                    stack.push((workflow, stage + 1, x, m, a, (*number, s.1)));
                }
                e => panic!("Unexpected variable: {e}"),
            },
        }
    }
    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_output() {
        let input = parse_input(get_input());
        assert_eq!(367602, part1(&input));
    }

    #[test]
    fn part2_output() {
        let input = parse_input(get_input());
        assert_eq!(125317461667458, part2(&input));
    }
}
