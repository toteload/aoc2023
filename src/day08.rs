use crate::util::lcm;

const START: u32 = 0;
const END: u32 = 2 * (25 + 26 * (25 + (26 * 25)));

fn parse_node(line: &str) -> (u32, u32, u32) {
    let bs = line.as_bytes();
    (
        2 * ((bs[0] - b'A') as u32 + 26 * ((bs[1] - b'A') as u32 + (26 * (bs[2] - b'A') as u32))),
        2 * ((bs[7] - b'A') as u32 + 26 * ((bs[8] - b'A') as u32 + (26 * (bs[9] - b'A') as u32))),
        2 * ((bs[12] - b'A') as u32
            + 26 * ((bs[13] - b'A') as u32 + (26 * (bs[14] - b'A') as u32))),
    )
}

pub fn part1(input: &str) -> u32 {
    let mut sections = input.split("\n\n");
    let instructions = sections
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .copied()
        .map(|x| if x == b'L' { 0u32 } else { 1u32 })
        .cycle();
    let nodes = sections.next().unwrap().lines().map(parse_node);

    let mut lookup = [0u32; 26 * 26 * 26 * 2];
    for (from, left, right) in nodes {
        lookup[from as usize] = left;
        lookup[(from | 1) as usize] = right;
    }

    let mut steps = 0;
    let mut at = START;
    for inst in instructions {
        if at == END {
            break;
        }

        at = lookup[(at | inst) as usize];

        steps += 1;
    }

    steps
}

pub fn part2(input: &str) -> i64 {
    let mut sections = input.split("\n\n");
    let instructions = sections
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .copied()
        .map(|x| if x == b'L' { 0u32 } else { 1u32 })
        .cycle();
    let nodes = sections.next().unwrap().lines().map(parse_node);

    let mut ghosts = Vec::new();
    let mut lookup = [0u32; 26 * 26 * 26 * 2];
    for (from, left, right) in nodes {
        lookup[from as usize] = left;
        lookup[(from | 1) as usize] = right;

        if (from / (2 * 26 * 26)) == 0 {
            ghosts.push(from);
        }
    }

    // There were some patterns in the input that I assume are true for everybodies input.
    // Based on some of these patterns optimizations are made.
    //
    // - Each ghost has only one candidate exit node.
    // - The period of each ghost is the same as the distance from the start to the exit node
    //   for each ghost.
    // - The period of each ghost is the product of two prime numbers. Each ghost has one of these
    //   primes in common.

    let mut answer = 1;

    for ghost in ghosts {
        let mut i = 0;
        let mut at = ghost;

        let insts = instructions.clone();

        // Walk around with a ghost until we hit an exit node.
        for inst in insts {
            if at / (2 * 26 * 26) == 25 {
                break;
            }

            at = lookup[(at | inst) as usize];

            i += 1;
        }

        answer = lcm(answer, i as i64);
    }

    answer
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(include_str!("../input/day_08.txt")), 15989);
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::part2(include_str!("../input/day_08.txt")),
            13830919117339
        );
    }
}
