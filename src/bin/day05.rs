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
    start_src: u64,
    start_dst: u64,
    len: u64,
}

impl Entry {
    fn src_range(&self) -> Range<u64> {
        self.start_src..self.start_src + self.len
    }

    fn src_end(&self) -> u64 {
        self.start_src + self.len
    }
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

fn map_range(map: &[Entry], range: &Range<u64>) -> Vec<Range<u64>> {
    let lo_idx = map.partition_point(|&Entry { start_src, .. }| start_src <= range.start);
    let hi_idx = map.partition_point(|&Entry { start_src, .. }| start_src <= range.end);

    //println!("{map:?}");
    //println!("{lo_idx}, {hi_idx}");

    let mut range = range.start..range.end;
    let mut res = Vec::new();

    assert!(lo_idx != 0);
    assert!(hi_idx != 0);

    if lo_idx == map.len() {
        return vec![range];
    }

    if hi_idx == map.len() {
        let e = map.last().unwrap();
        let start = e.start_src + e.len;
        return vec![start..range.end];
    }
                //println!("{map:?}, {range:?}, {lo_idx}, {hi_idx}");

    for at in (lo_idx-1)..hi_idx {
        if range.is_empty() {
            break;
        }

        if !(map[at].src_range().contains(&range.start)) {
            println!("{map:?}, {range:?}, {lo_idx}, {hi_idx}");
        }

        assert!(map[at].src_range().contains(&range.start));

        //if !map[at].src_range().contains(&range.start) {
        //    if !(map[at].start_src > range.start) {
        //        println!("{map:?}, {range:?}, {lo_idx}, {hi_idx}");
        //    }

        //    res.push(range.start..map[at].start_src);
        //    range = map[at].start_src..range.end;
        //}

        let end = range.end.min(map[at].src_end());
        res.push(
            (range.start + map[at].start_dst - map[at].start_src)
                ..(end + map[at].start_dst - map[at].start_src),
        );
        range = end..range.end;
    }

    res
}

fn fill_gaps(map: Vec<Entry>) -> Vec<Entry> {
    let n = map.len();
    let mut res = Vec::new();

    for i in 1..n {
        if map[i-1].start_src+map[i-1].len < map[i].start_src {
            res.push(Entry {
                start_src: map[i-1].start_src+map[i-1].len,
                start_dst: map[i-1].start_src+map[i-1].len,
                len: map[i].start_src - map[i-1].start_src+map[i-1].len,
            });
        }
    }

    res.extend(map);
    res.sort_by_key(|e| e.start_src);

    res
}

fn part_2(input: &str) {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let seeds = parse_nums(sections[0]);

    let seed_ranges = seeds
        .chunks_exact(2)
        .map(|nums| {
            let &[start, len] = nums else { unreachable!() };
            start..(start+len)
        })
        .collect::<Vec<_>>();

    let maps = sections[1..]
        .iter()
        .map(|&s| fill_gaps(parse_map(s)))
        .collect::<Vec<_>>();

    //println!("{:?}", map_range(&maps[1], &(14..50)));

    println!("{seed_ranges:?}");

    let mut last_ranges = seed_ranges;
    for map in maps {
        let mut out = Vec::new();
        for r in last_ranges {
            out.extend(map_range(&map, &r));
        }
        println!("{out:?}");
        last_ranges = out;
    }

    //println!("{last_ranges:?}");

    //let answer = last_ranges.iter().map(|r| r.start).min().unwrap();

    //println!("{answer}");
}

fn main() {
    part_1(INPUT);
    part_2(INPUT);
}
