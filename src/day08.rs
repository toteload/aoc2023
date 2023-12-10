use crate::util::lcm;
use std::collections::{HashMap, HashSet};

fn parse_node(line: &str) -> (&[u8], &[u8], &[u8]) {
    let bs = line.as_bytes();
    (&bs[..3], &bs[7..10], &bs[12..15])
}

pub fn part1(input: &str) -> u32 {
    let mut sections = input.split("\n\n");
    let instructions = sections.next().unwrap().as_bytes().iter().copied().cycle();
    let nodes = sections.next().unwrap().lines().map(parse_node);

    let mut lookup = HashMap::new();
    for (from, left, right) in nodes {
        lookup.insert((from, b'L'), left);
        lookup.insert((from, b'R'), right);
    }

    let mut steps = 0;
    let mut at: &[u8] = b"AAA";
    for inst in instructions {
        if at == b"ZZZ" {
            break;
        }

        at = lookup.get(&(at, inst)).unwrap();

        steps += 1;
    }

    steps
}

pub fn part2(input: &str) -> i64 {
    let mut sections = input.split("\n\n");
    let instruction_bytes = sections.next().unwrap().as_bytes();
    let instruction_count = instruction_bytes.len();
    let instructions = instruction_bytes.iter().copied().cycle();
    let nodes = sections.next().unwrap().lines().map(parse_node);

    let mut ghosts = Vec::new();
    let mut lookup = HashMap::new();
    for (from, left, right) in nodes {
        lookup.insert((from, b'L'), left);
        lookup.insert((from, b'R'), right);

        if from[2] == b'A' {
            ghosts.push(from);
        }
    }

    let mut periods = Vec::new();

    for ghost in ghosts.iter() {
        let mut i = 0;
        let mut at = ghost;
        let mut history = Vec::new();
        let mut visited = HashSet::new();

        let insts = instructions.clone();

        // Walk around with a ghost and stop when we find a node where we have been before with the
        // same instruction index.
        for inst in insts {
            if !visited.insert((at, i)) {
                break;
            }

            history.push((at, i));

            at = lookup.get(&(at, inst)).unwrap();
            i = (i + 1) % instruction_count;
        }

        // For my input the number of steps it takes to reach an exit node happens to be the same
        // as the size of period. I assume that this is the case for everyone.
        let offset = history.iter().position(|p| p == &(at, i)).unwrap();
        let period = history.len() - offset;

        periods.push(period as i64);
    }

    periods.iter().copied().reduce(lcm).unwrap()
}
