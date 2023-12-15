use ahash::AHashMap;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::collections::HashMap;
use std::iter;

fn pack(record: u64, groups: u64) -> u64 {
    record | groups << 40
}

fn parse_line2(line: &str) -> u64 {
    let mut sections = line.split_ascii_whitespace();
    let record = sections.next().unwrap().as_bytes();
    let record = record.iter().enumerate().fold(0u64, |acc, (i, c)| {
        let x = match c {
            b'.' => DOT,
            b'#' => HASHTAG,
            b'?' => QUESTION,
            _ => 0,
        };
        acc | (x << (2 * i))
    });

    let groups = sections
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .enumerate()
        .fold(0u64, |acc, (i, x)| acc | ((x as u64) << (4 * i)));

    //println!("parse: record {:b}, groups {:b}", record, groups);

    pack(record, groups)
}

// Biggest record had a length of 20
// Most groups for one entry was 6
// Biggest group was 15
//
// `groups` can comfortably be packed in a u32 using 4 bits per group size
// A record can comfortably be packed in a u64 using 2 bits per position
//
// `record` is 40 bits and `groups` is 24 `bits`

const NIL: u64 = 0;
const HASHTAG: u64 = 1;
const DOT: u64 = 2;
const QUESTION: u64 = 3;

fn count2(memo: &mut AHashMap<u64, u64>, entry: u64) -> u64 {
    if let Some(count) = memo.get(&entry) {
        return *count;
    }

    let (record, groups) = (entry & 0xffffffffff, entry >> 40);

    let n = groups & 0xf;
    let count = match record & 0x3 {
        NIL if groups == 0 => 1,
        NIL => 0,
        HASHTAG if n == 0 => 0,

        DOT | QUESTION if n == 0 => count2(memo, pack(record >> 2, groups >> 4)),
        DOT => count2(memo, pack(record >> 2, groups)),

        c @ (QUESTION | HASHTAG) => 'blk: {
            let count = if c == QUESTION {
                count2(memo, pack(record >> 2, groups))
            } else {
                0
            };

            for i in 0..n {
                let c = (record >> (i * 2)) & 0x1;
                // If this is NIL or DOT
                if c == 0 {
                    break 'blk count;
                }
            }

            count + count2(memo, pack(record >> (2 * n), groups & !0xf))
        }

        _ => unreachable!(),
    };

    memo.insert(entry, count);

    count
}
pub fn part1(input: &str) -> u64 {
    let mut answer = 0;
    for line in input.lines() {
        answer += {
            let mut memo = AHashMap::new();
            let entry = parse_line2(line);
            count2(&mut memo, entry)
        };
    }

    answer
}

fn parse_line(line: &str) -> (&[u8], Vec<u8>) {
    let mut sections = line.split_ascii_whitespace();
    let record = sections.next().unwrap().as_bytes();
    let groups = sections
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    (record, groups)
}

fn count_arrangements<'a>(
    memo: &mut AHashMap<(&'a [u8], &'a [u8]), u64>,
    record: &'a [u8],
    groups: &'a [u8],
) -> u64 {
    if let Some(count) = memo.get(&(record, groups)) {
        return *count;
    }

    let precord = record;
    let pgroups = groups;

    let mut record = record;
    let mut groups = groups;

    while !record.is_empty() {
        let start = record[0];

        match start {
            b'.' => {
                let leading_dot_count = record
                    .iter()
                    .position(|&c| c != b'.')
                    .unwrap_or(record.len());

                record = &record[leading_dot_count..];
            }

            b'#' => {
                if groups.is_empty() {
                    return 0;
                }

                let leading_hashtag_count = record
                    .iter()
                    .position(|&c| c != b'#')
                    .unwrap_or(record.len());

                if (leading_hashtag_count as u8) == groups[0] {
                    groups = &groups[1..];
                    let start = (leading_hashtag_count + 1).min(record.len());
                    record = &record[start..];
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    match (record, groups) {
        ([], []) => return 1,
        ([], _) => return 0,
        (_, []) => {
            for c in record {
                if *c == b'#' {
                    return *memo.entry((precord, pgroups)).or_insert(0);
                }
            }

            return *memo.entry((precord, pgroups)).or_insert(1);
        }
        _ => (),
    }

    debug_assert!(matches!(record[0], b'?' | b'#'));

    let mut answer = 0;

    if record[0] == b'?' {
        answer += count_arrangements(memo, &record[1..], groups);
    }

    answer += 'blk: {
        let (head, groups) = {
            let [head, rest @ ..] = groups else {
                unreachable!()
            };
            (*head, rest)
        };

        if (head as usize) > record.len() {
            break 'blk 0;
        }

        for i in 1..(head as usize) {
            if record[i] == b'.' {
                break 'blk 0;
            }
        }

        if (head as usize) < record.len() && record[head as usize] == b'#' {
            break 'blk 0;
        }

        if record.len() > (head + 1) as usize {
            count_arrangements(memo, &record[(head as usize + 1)..], groups)
        } else {
            count_arrangements(memo, &[], groups)
        }
    };

    *memo.entry((precord, pgroups)).or_insert(answer)
}

pub fn part2(input: &str) -> u64 {
    let mut answer = 0;
    for line in input.lines() {
        let (record, mut groups) = parse_line(line);

        let mut record = record.to_vec();

        let record_len = record.len();
        let groups_len = groups.len();

        for _ in 0..4 {
            record.push(b'?');
            record.extend_from_within(..record_len);
            groups.extend_from_within(..groups_len);
        }

        let mut memo = AHashMap::new();
        answer += count_arrangements(&mut memo, &record, &groups);
    }

    answer
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(include_str!("../input/day_12.txt")), 7939);
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::part2(include_str!("../input/day_12.txt")),
            850504257483930
        );
    }
}
