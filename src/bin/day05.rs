use core::ops::Range;

const INPUT: &str = include_str!("../../input/day_05.txt");

fn parse_nums(section: &str) -> Vec<u64> {
    let start = section.bytes().position(|b| b == b':').unwrap();
    let list = &section[start + 1..].trim();
    list.split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

#[derive(Debug)]
struct Entry {
    pub src: Range<u64>,
    pub dst: Range<u64>,
}

impl Entry {
    fn new(src_start: u64, dst_start: u64, len: u64) -> Entry {
        Entry {
            src: src_start..(src_start + len),
            dst: dst_start..(dst_start + len),
        }
    }

    fn transform(&self, x: u64) -> u64 {
        self.dst.start + x - self.src.start
    }
}

fn parse_map(section: &str) -> Vec<Entry> {
    let mut entries = parse_nums(section)
        .chunks_exact(3)
        .map(|nums| {
            let &[start_dst, start_src, len] = nums else {
                unreachable!()
            };

            Entry::new(start_src, start_dst, len)
        })
        .collect::<Vec<_>>();

    entries.sort_by_key(|e| e.src.start);

    entries
}

fn fill_gaps(map: Vec<Entry>) -> Vec<Entry> {
    let n = map.len();
    let mut res = Vec::new();

    // Fill in any gaps between the ranges present in the map
    for i in 1..n {
        let prev = &map[i - 1];
        let curr = &map[i];
        if prev.src.end < curr.src.start {
            res.push(Entry::new(
                prev.src.end,
                prev.src.end,
                curr.src.start - prev.src.end,
            ));
        }
    }

    if map[0].src.start > 0 {
        res.push(Entry::new(0, 0, map[0].src.start));
    }

    let Some(last) = map.last() else {
        unreachable!()
    };

    if last.src.end < (u64::MAX - 1) {
        res.push(Entry::new(
            last.src.end,
            last.src.end,
            u64::MAX - last.src.end,
        ));
    }

    res.extend(map);
    res.sort_by_key(|e| e.src.start);

    res
}

fn seed_to_location(maps: &[Vec<Entry>], seed: u64) -> u64 {
    let mut x = seed;

    for map in maps {
        let entry_idx = map.partition_point(
            |&Entry {
                 src: Range { start, .. },
                 ..
             }| start <= x,
        );

        assert!(entry_idx != 0 && entry_idx != map.len());

        let entry = &map[entry_idx - 1];

        x = entry.transform(x);
    }

    x
}

fn part_1(input: &str) {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let seeds = parse_nums(sections[0]);
    let maps = sections[1..]
        .iter()
        .map(|&s| parse_map(s))
        .map(fill_gaps)
        .collect::<Vec<_>>();

    let answer = seeds
        .iter()
        .map(|&seed| seed_to_location(&maps, seed))
        .min()
        .unwrap();

    assert_eq!(answer, 525792406);
}

fn map_range(map: &[Entry], range: &Range<u64>) -> Vec<Range<u64>> {
    let lo_idx = map.partition_point(
        |&Entry {
             src: Range { start, .. },
             ..
         }| start <= range.start,
    ) - 1;

    let hi_idx = map.partition_point(
        |&Entry {
             src: Range { start, .. },
             ..
         }| start <= range.end,
    );

    let mut range = range.start..range.end;
    let mut res = Vec::new();

    for entry in &map[lo_idx..hi_idx] {
        if range.is_empty() {
            break;
        }

        let end = range.end.min(entry.src.end);
        res.push(entry.transform(range.start)..entry.transform(end));
        range = end..range.end;
    }

    res
}

fn part_2(input: &str) {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let seeds = parse_nums(sections[0]);

    let seed_ranges = seeds
        .chunks_exact(2)
        .map(|nums| {
            let &[start, len] = nums else { unreachable!() };
            start..(start + len)
        })
        .collect::<Vec<_>>();

    let maps = sections[1..]
        .iter()
        .map(|&s| parse_map(s))
        .map(fill_gaps)
        .collect::<Vec<_>>();

    let mut last_ranges = seed_ranges;
    for map in maps {
        let mut out = Vec::new();
        for r in last_ranges {
            out.extend(map_range(&map, &r));
        }
        last_ranges = out;
    }

    let answer = last_ranges.iter().map(|r| r.start).min().unwrap();
    assert_eq!(answer, 79004094);
}

fn main() {
    part_1(INPUT);
    part_2(INPUT);
}
