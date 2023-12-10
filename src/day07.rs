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

fn determine_hand_type(distinct_card_count: u32, max_same_card_count: u32) -> HandType {
    use HandType::*;

    match (distinct_card_count, max_same_card_count) {
        (5, _) => HighCard,
        (4, _) => OnePair,
        (3, 2) => TwoPair,
        (3, 3) => ThreeOfAKind,
        (2, 3) => FullHouse,
        (2, 4) => FourOfAKind,
        (1, _) => FiveOfAKind,
        _ => unreachable!(),
    }
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

    let mut distinct_card_count = 0;
    let mut max_same_card_count = 0;
    for c in counter {
        if c > 0 {
            distinct_card_count += 1;
        }

        max_same_card_count = max_same_card_count.max(c);
    }

    let ty = determine_hand_type(distinct_card_count, max_same_card_count);

    (Hand { ty, dealt }, bid)
}

pub fn part1(input: &str) -> u64 {
    let mut entries = input.lines().map(parse_entry).collect::<Vec<_>>();
    entries.sort_unstable_by_key(|entry| entry.0);

    entries
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum::<u64>()
}

fn card_value_with_joker(c: u8) -> u32 {
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

fn determine_hand_type_with_jokers(
    distinct_card_count: u32,
    max_same_card_count: u32,
    joker_count: u32,
) -> HandType {
    use HandType::*;

    match (distinct_card_count, max_same_card_count, joker_count) {
        (_, _, 0) => determine_hand_type(distinct_card_count, max_same_card_count),

        (4, _, 1) => OnePair,
        (3, _, 1) => ThreeOfAKind,
        (2, 2, 1) => FullHouse,
        (2, 3, 1) => FourOfAKind,

        (3, _, 2) => ThreeOfAKind,
        (2, _, 2) => FourOfAKind,

        (2, _, 3) => FourOfAKind,

        _ => FiveOfAKind,
    }
}

fn parse_entry_part2(line: &str) -> (Hand, u64) {
    let mut sections = line.split_ascii_whitespace();
    let hand = sections.next().unwrap();
    let bid = sections.next().unwrap().parse::<u64>().unwrap();

    let dealt = <&[u8; 5]>::try_from(hand.as_bytes())
        .unwrap()
        .map(card_value_with_joker);

    let mut counter = [0; 15];
    for i in dealt {
        counter[i as usize] += 1;
    }

    let mut distinct_card_count = 0;
    let mut max_same_card_count = 0;
    for c in &counter[2..] {
        if *c > 0 {
            distinct_card_count += 1;
        }

        max_same_card_count = max_same_card_count.max(*c);
    }

    let joker_count = counter[1];

    let ty = determine_hand_type_with_jokers(distinct_card_count, max_same_card_count, joker_count);

    (Hand { ty, dealt }, bid)
}

pub fn part2(input: &str) -> u64 {
    let mut entries = input.lines().map(parse_entry_part2).collect::<Vec<_>>();
    entries.sort_unstable_by_key(|entry| entry.0);

    entries
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum::<u64>()
}
