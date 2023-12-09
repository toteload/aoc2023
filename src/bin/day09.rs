mod util;
use util::parse_whitespace_separated_items;

const INPUT: &str = include_str!("../../input/day_09.txt");

fn next_value(seq: &[i64]) -> i64 {
    let &[.., a, b] = seq else { unreachable!() };

    if a == 0 && b == 0 {
        return 0;
    }

    let diffs = seq.windows(2).map(|xs| xs[1] - xs[0]).collect::<Vec<_>>();

    b + next_value(&diffs)
}

fn part_1(input: &str) {
    let history = input.lines().map(parse_whitespace_separated_items::<i64>).collect::<Vec<_>>();
    let answer = history.iter().map(|v| next_value(v.as_slice())).sum::<i64>();
    assert_eq!(answer, 1641934234);
}

fn prev_value(seq: &[i64]) -> i64 {
    let &[a, b, ..] = seq else { unreachable!() };

    if a == 0 && b == 0 {
        return 0;
    }

    let diffs = seq.windows(2).map(|xs| xs[1] - xs[0]).collect::<Vec<_>>();

    a - prev_value(&diffs)
}

fn part_2(input: &str) {
    let history = input.lines().map(parse_whitespace_separated_items::<i64>).collect::<Vec<_>>();
    let answer = history.iter().map(|v| prev_value(v.as_slice())).sum::<i64>();
    assert_eq!(answer, 975);
}

fn main() {
    part_1(INPUT);
    part_2(INPUT);
}
