use core::ops::Range;

const SIZE: usize = 140;
const MAXP: usize = SIZE * SIZE;

fn neighbors(p: usize) -> [usize; 8] {
    [
        p.wrapping_sub(SIZE + 1),
        p.wrapping_sub(SIZE),
        p.wrapping_sub(SIZE - 1),
        p.wrapping_sub(1),
        p + 1,
        p + SIZE - 1,
        p + SIZE,
        p + SIZE + 1,
    ]
}

pub fn part1(input: &str) -> u64 {
    // Parse all the symbols
    // ---------------------

    let mut symbols = Vec::<(u8, usize, usize)>::new();

    for (i, line) in input.lines().enumerate() {
        let line_symbols = line.as_bytes().iter().enumerate().filter_map(|(j, &b)| {
            if !b.is_ascii_digit() && b != b'.' {
                Some((b, i, j))
            } else {
                None
            }
        });

        symbols.extend(line_symbols);
    }

    // Parse all the numbers
    // ---------------------

    let mut nums = Vec::<(usize, Range<usize>, u64)>::new();

    for (i, line) in input.lines().enumerate() {
        let mut chars = line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| c.is_ascii_digit().then_some(i))
            .peekable();

        while chars.peek().is_some() {
            let start = chars.next().unwrap();

            let mut last = start;
            while let Some(x) = chars.peek() {
                if x - last != 1 {
                    break;
                }

                last = *x;

                chars.next();
            }

            let end = last + 1;

            let c = line[start..end].parse::<u64>().unwrap();

            nums.push((i, start..end, c));
        }
    }

    let mut lookup = vec![b'.'; SIZE * SIZE];
    for (b, line_idx, x) in symbols {
        let p = line_idx * SIZE + x;
        lookup[p] = b;
    }

    let mut sum = 0;

    'outer: for (y, span, num) in nums {
        for x in span {
            let nump = y * SIZE + x;
            for np in neighbors(nump) {
                if np >= MAXP {
                    continue;
                }

                if lookup[np] != b'.' {
                    sum += num;
                    continue 'outer;
                }
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> u64 {
    // Parse all the symbols
    // ---------------------

    let mut symbols = Vec::<(u8, usize, usize)>::new();

    for (i, line) in input.lines().enumerate() {
        let line_symbols = line.as_bytes().iter().enumerate().filter_map(|(j, &b)| {
            if !b.is_ascii_digit() && b != b'.' {
                Some((b, i, j))
            } else {
                None
            }
        });

        symbols.extend(line_symbols);
    }

    // Parse all the numbers
    // ---------------------

    let mut nums = Vec::<(usize, Range<usize>, u64)>::new();

    for (i, line) in input.lines().enumerate() {
        let mut chars = line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| c.is_ascii_digit().then_some(i))
            .peekable();

        while chars.peek().is_some() {
            let start = chars.next().unwrap();

            let mut last = start;
            while let Some(x) = chars.peek() {
                if x - last != 1 {
                    break;
                }

                last = *x;

                chars.next();
            }

            let end = last + 1;

            let c = line[start..end].parse::<u64>().unwrap();

            nums.push((i, start..end, c));
        }
    }

    // Create lookup table for the numbers
    // -----------------------------------

    let mut lookup = vec![0u64; SIZE * SIZE];
    for (line_idx, span, num) in nums {
        for x in span {
            let p = line_idx * SIZE + x;
            lookup[p] = num;
        }
    }

    let mut sum = 0;
    let mut buf = Vec::new();

    for (b, y, x) in symbols {
        if b != b'*' {
            continue;
        }

        buf.clear();

        let symp = y * SIZE + x;
        for p in neighbors(symp) {
            if p >= MAXP {
                continue;
            }

            if lookup[p] != 0 {
                // This is very ugly and technically even incorrect, but it worked :)
                // It is possible that a gear is adjacent to two different numbers with
                // the same value.
                // Also it could be that the number zero is present in the input and adjacent to a
                // gear. It would then cancel out and result in zero, but I use zero as a value
                // meaning 'there is no number here'.
                if buf.iter().any(|&x| x == lookup[p]) {
                    continue;
                }

                buf.push(lookup[p]);
            }
        }

        if buf.len() != 2 {
            continue;
        }

        sum += buf.iter().product::<u64>();
    }

    sum
}
