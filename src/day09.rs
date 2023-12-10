use crate::util::parse_whitespace_separated_items;

fn next_value(seq: &[i64]) -> i64 {
    let &[.., a, b] = seq else { unreachable!() };

    if a == 0 && b == 0 {
        return 0;
    }

    let diffs = seq.windows(2).map(|xs| xs[1] - xs[0]).collect::<Vec<_>>();

    b + next_value(&diffs)
}

pub fn part1(input: &str) -> i64 {
    let history = input
        .lines()
        .map(parse_whitespace_separated_items::<i64>)
        .collect::<Vec<_>>();
    history
        .iter()
        .map(|v| next_value(v.as_slice()))
        .sum::<i64>()
}

fn prev_value(seq: &[i64]) -> i64 {
    let &[a, b, ..] = seq else { unreachable!() };

    if a == 0 && b == 0 {
        return 0;
    }

    let diffs = seq.windows(2).map(|xs| xs[1] - xs[0]).collect::<Vec<_>>();

    a - prev_value(&diffs)
}

pub fn part2(input: &str) -> i64 {
    let history = input
        .lines()
        .map(parse_whitespace_separated_items::<i64>)
        .collect::<Vec<_>>();
    history
        .iter()
        .map(|v| prev_value(v.as_slice()))
        .sum::<i64>()
}
