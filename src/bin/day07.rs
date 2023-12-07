use std::cmp::Ordering;

const INPUT: &str = include_str!("../../input/day_07.txt");

fn card_value(c: u8) -> u32 {
    match c {
        b'1'..=b'9' => (c - b'1') as u32,
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
    }
}

#[derive(PartialEq)]
struct Hand([u32; 5]);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let isplit_idxs = self
            .0
            .windows(2)
            .enumerate()
            .filter_map(|(i, xs)| (xs[0] != xs[1]).then_some(i));

        let jsplit_idxs = self
            .0
            .windows(2)
            .enumerate()
            .filter_map(|(i, xs)| (xs[0] != xs[1]).then_some(i));

        for (i, j) in isplit_idxs.zip(jsplit_idxs) {
            if i != j {
                todo!()
            }
        }

        for (a, b) in self.0.iter().zip(other.0.iter()) {
            if a != b {
                todo!()
            }
        }

        todo!()
    }
}

fn parse_hand(line: &str) -> ([u32; 5], u32) {
    let sections = line.split_ascii_whitespace();
    let hand = sections.next().unwrap();
    let bet = sections.next().unwrap().parse::<u32>();
    hand.bytes().map(card_value)

    // TODO also order the cards
    todo!()
}

fn part_1(input: &str) {
    let hands = input.lines().map(parse_hand);

    todo!()
}

fn part_2(_input: &str) {
    todo!()
}

fn main() {
    part_1(INPUT);
    part_2(INPUT);
}
