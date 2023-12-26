use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn len(&self) -> usize {
        (self.end - self.start) as usize
    }

    fn split(&self, mid: u64) -> (Self, Self) {
        (
            Self {
                start: self.start,
                end: mid,
            },
            Self {
                start: mid,
                end: self.end,
            },
        )
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    GreaterThan,
    LessThan,
}

type Part = [u32; 4];

#[derive(Debug, Clone, Copy)]
enum Action<'a> {
    Accept,
    Reject,
    Jump(&'a str),
}

#[derive(Debug, Clone, Copy)]
struct Conditional<'a> {
    category: u8,
    op: Op,
    limit: u32,
    action: Action<'a>,
}

#[derive(Debug)]
enum Rule<'a> {
    ByDefault(Action<'a>),
    Condition(Conditional<'a>),
}

type Workflow<'a> = Vec<Rule<'a>>;

fn parse_action(action: &str) -> Action {
    match action {
        "A" => Action::Accept,
        "R" => Action::Reject,
        x => Action::Jump(x),
    }
}

fn parse_rule(rule: &str) -> Rule {
    let bs = rule.as_bytes();

    if rule.len() < 2 || !matches!(bs[1], b'>' | b'<') {
        return Rule::ByDefault(parse_action(rule));
    }

    let category = match bs[0] {
        b'x' => 0,
        b'm' => 1,
        b'a' => 2,
        b's' => 3,
        _ => unreachable!(),
    };

    let op = match bs[1] {
        b'>' => Op::GreaterThan,
        b'<' => Op::LessThan,
        _ => unreachable!(),
    };

    let mut parts = (&bs[2..]).split(|&c| c == b':');
    let limit = std::str::from_utf8(parts.next().unwrap())
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let action = parse_action(std::str::from_utf8(parts.next().unwrap()).unwrap());

    Rule::Condition(Conditional {
        category,
        op,
        limit,
        action,
    })
}

fn parse_workflow(line: &str) -> (&str, Workflow) {
    let mut parts = line.split('{');
    let name = parts.next().unwrap();
    let rules_text = {
        let text = parts.next().unwrap();
        &text[..text.len() - 1]
    };

    let rules = rules_text.split(',').map(parse_rule).collect::<Vec<_>>();

    (name, rules)
}

fn parse_part(line: &str) -> Part {
    let bs = line.as_bytes();
    let bs = &bs[1..bs.len() - 1];

    let mut values = [0; 4];

    for (i, category) in bs.split(|&c| c == b',').enumerate() {
        let x = std::str::from_utf8(&category[2..])
            .unwrap()
            .parse::<u32>()
            .unwrap();
        values[i] = x;
    }

    values
}

pub fn part1(input: &str) -> u64 {
    let mut sections = input.split("\n\n");

    let workflows = sections
        .next()
        .unwrap()
        .lines()
        .map(parse_workflow)
        .collect::<HashMap<_, _>>();

    let parts = sections
        .next()
        .unwrap()
        .lines()
        .map(parse_part)
        .collect::<Vec<_>>();

    let mut answer = 0;
    for part in parts {
        let mut current = "in";
        let accepted = 'check: loop {
            let rules = workflows.get(current).unwrap();
            for rule in rules {
                let action = match rule {
                    Rule::ByDefault(action) => Some(action),
                    Rule::Condition(Conditional {
                        category,
                        op,
                        limit,
                        action,
                    }) => {
                        let value = part[*category as usize];

                        let matched = match op {
                            Op::GreaterThan => value > *limit,
                            Op::LessThan => value < *limit,
                        };

                        matched.then_some(action)
                    }
                };

                if let Some(action) = action {
                    match action {
                        Action::Accept => break 'check true,
                        Action::Reject => break 'check false,
                        Action::Jump(label) => {
                            current = label;
                            continue 'check;
                        }
                    }
                }
            }
        };

        if accepted {
            let score = part.iter().sum::<u32>();
            answer += score as u64;
        }
    }

    answer
}

fn part_configuration_count(part: &[Range; 4]) -> u64 {
    part.iter().map(|x| x.len() as u64).product::<u64>()
}

fn count_combinations(workflows: &HashMap<&str, Workflow>, label: &str, part: [Range; 4]) -> u64 {
    if part_configuration_count(&part) == 0 {
        return 0;
    }

    let mut count = 0;
    let rules = workflows.get(label).unwrap();
    let mut part = part;
    for rule in rules {
        use Action::*;
        use Rule::*;

        match rule {
            ByDefault(Accept) => count += part_configuration_count(&part),
            ByDefault(Reject) => (),
            ByDefault(Jump(next)) => count += count_combinations(workflows, next, part),

            Condition(Conditional {
                category,
                op,
                limit,
                action,
            }) => {
                let idx = *category as usize;
                let parts_if_matched = match op {
                    Op::GreaterThan => {
                        let mut res = part;
                        (part[idx], res[idx]) = part[idx].split(*limit as u64 + 1);
                        res
                    }
                    Op::LessThan => {
                        let mut res = part;
                        (res[idx], part[idx]) = part[idx].split(*limit as u64);
                        res
                    }
                };

                match action {
                    Accept => count += part_configuration_count(&parts_if_matched),
                    Reject => (),
                    Jump(next) => count += count_combinations(workflows, next, parts_if_matched),
                }
            }
        }
    }

    count
}

pub fn part2(input: &str) -> u64 {
    let workflows = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .lines()
        .map(parse_workflow)
        .collect::<HashMap<_, _>>();

    count_combinations(&workflows, "in", [Range::new(1, 4001); 4])
}
