const INPUT: &str = include_str!("../../input/day_05.txt");

fn parse_nums(section: &str) -> Vec<u64> {
    let start = section.bytes().position(|b| b == b':').unwrap();
    let list = &section[start + 1..].trim();
    list.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect()
}

#[derive(Debug)]
struct Entry {
    start_src: u64,
    start_dst: u64,
    len: u64,
}

fn parse_map(section: &str) -> Vec<Entry> {
    let mut entries = parse_nums(section)
        .chunks_exact(3)
        .map(|nums| {
            let &[start_dst, start_src, len] = nums else {
                unreachable!()
            };

            Entry {
                start_src,
                start_dst,
                len,
            }
        })
        .collect::<Vec<_>>();

    entries.sort_by_key(|e| e.start_src);

    entries
}

fn seed_to_location(maps: &[Vec<Entry>], seed: u64) -> u64 {
    let mut x = seed;

    for map in maps {
        let entry_idx = map.partition_point(|&Entry { start_src, .. }| start_src <= x);

        if entry_idx == 0 {
            continue;
        }

        let entry = &map[entry_idx - 1];
        let src_range = entry.start_src..(entry.start_src + entry.len);
        if !src_range.contains(&x) {
            continue;
        }

        x = entry.start_dst + (x - entry.start_src);
    }

    x
}

fn part_1(input: &str) {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let seeds = parse_nums(sections[0]);
    let maps = sections[1..]
        .iter()
        .map(|&s| parse_map(s))
        .collect::<Vec<_>>();

    let answer = seeds
        .iter()
        .map(|&seed| seed_to_location(&maps, seed))
        .min()
        .unwrap();

    println!("{answer}");
}

fn part_2(input: &str) {
    todo!()
}

fn main() {
    part_1(INPUT);
    part_2(INPUT);
}
