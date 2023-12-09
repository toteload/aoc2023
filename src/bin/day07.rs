const INPUT: &str = include_str!("../../input/day_07.txt");

fn card_value(c: u8) -> u32 {
    match c {
        b'2'..=b'9' => (c - b'0') as u32,
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => unreachable!(),
    }
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Hand {
    ty: HandType,
    dealt: [u32; 5],
}

fn parse_entry(line: &str) -> (Hand, u64) {
    let mut sections = line.split_ascii_whitespace();
    let hand = sections.next().unwrap();
    let bid = sections.next().unwrap().parse::<u64>().unwrap();

    let dealt = <&[u8; 5]>::try_from(hand.as_bytes())
        .unwrap()
        .map(card_value);

    let mut counter = [0; 15];
    for i in dealt {
        counter[i as usize] += 1;
    }

    let mut group_count = 0;
    let mut biggest_group_size = 0;
    for c in counter {
        if c > 0 {
            group_count += 1;
        }

        biggest_group_size = biggest_group_size.max(c);
    }

    use HandType::*;

    let ty = match (group_count, biggest_group_size) {
        (5, _) => HighCard,
        (4, _) => OnePair,
        (3, 2) => TwoPair,
        (3, 3) => ThreeOfAKind,
        (2, 3) => FullHouse,
        (2, 4) => FourOfAKind,
        (1, _) => FiveOfAKind,
        _ => unreachable!(),
    };

    (Hand { ty, dealt }, bid)
}

fn part_1(input: &str) {
    let mut entries = input.lines().map(parse_entry).collect::<Vec<_>>();
    entries.sort_unstable_by_key(|entry| entry.0);

    let answer = entries
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum::<u64>();

    println!("{answer}");
}

fn card_value2(c: u8) -> u32 {
    match c {
        b'J' => 1,
        b'2'..=b'9' => (c - b'0') as u32,
        b'T' => 10,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => unreachable!(),
    }
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType2 {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Hand2 {
    ty: HandType2,
    dealt: [u32; 5],
}

fn find_best_joker_usage(group_count: u32, biggest_group_size: u32, joker_count: u32) -> (u32, u32) {
    let n = group_count;
    let s = biggest_group_size;
    let j = joker_count;

    match (n, s, j) {
        (_, _, 0) => (n, s),

        (4, _, 1) => (4, 2),
        (3, _, 1) => (3, 3),
        (2, 2, 1) => (2, 3),
        (2, 3, 1) => (2, 4),

        (3, _, 2) => (3, 3),
        (2, _, 2) => (2, 4),

        (2, _, 3) => (2, 4),

        _ => (1, 5),
    }
}

fn parse_entry2(line: &str) -> (Hand2, u64) {
    let mut sections = line.split_ascii_whitespace();
    let hand = sections.next().unwrap();
    let bid = sections.next().unwrap().parse::<u64>().unwrap();

    let dealt = <&[u8; 5]>::try_from(hand.as_bytes())
        .unwrap()
        .map(card_value2);

    let mut counter = [0; 15];
    for i in dealt {
        counter[i as usize] += 1;
    }

    let mut group_count = 0;
    let mut biggest_group_size = 0;
    for c in &counter[2..] {
        if *c > 0 {
            group_count += 1;
        }

        biggest_group_size = biggest_group_size.max(*c);
    }

    let joker_count = counter[1];

    use HandType2::*;

    let x = find_best_joker_usage(group_count, biggest_group_size, joker_count);

    let ty = match x {
        (5, _) => HighCard,
        (4, _) => OnePair,
        (3, 2) => TwoPair,
        (3, 3) => ThreeOfAKind,
        (2, 3) => FullHouse,
        (2, 4) => FourOfAKind,
        (1, _) => FiveOfAKind,
        _ => unreachable!(),
    };

    (Hand2 { ty, dealt }, bid)
}

fn part_2(input: &str) {
    let mut entries = input.lines().map(parse_entry2).collect::<Vec<_>>();
    entries.sort_unstable_by_key(|entry| entry.0);

    let answer = entries
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum::<u64>();

    println!("{answer}");
}

fn main() {
    part_1(INPUT);
    part_2(INPUT);
}
