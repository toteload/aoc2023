struct Bitmap {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

fn parse_pattern(pattern: &str) -> Bitmap {
    let lines = pattern
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let rows = lines
        .iter()
        .map(|line| {
            line.iter().enumerate().fold(0, |acc, (i, c)| {
                let x = if *c == b'.' { 0 } else { 1 };
                acc | x << i
            })
        })
        .collect::<Vec<u32>>();

    let cols = (0..lines[0].len())
        .map(|i| {
            lines
                .iter()
                .map(|line| line[i])
                .enumerate()
                .fold(0, |acc, (i, c)| {
                    let x = if c == b'.' { 0 } else { 1 };
                    acc | x << i
                })
        })
        .collect::<Vec<u32>>();

    Bitmap { rows, cols }
}

fn count_reflections(lines: &[u32]) -> u32 {
    for i in 1..lines.len() {
        let n = i.min(lines.len() - i);
        let (left, right) = &lines[i - n..i + n].split_at(n);
        if left.iter().eq(right.iter().rev()) {
            return i as u32;
        }
    }

    0
}

pub fn part1(input: &str) -> u32 {
    let patterns = input.split("\n\n");

    patterns
        .map(parse_pattern)
        .map(|bitmap| {
            let a = count_reflections(&bitmap.cols);
            let b = count_reflections(&bitmap.rows);
            a + 100 * b
        })
        .sum::<u32>()
}

fn count_smudged_reflections(lines: &[u32]) -> u32 {
    'search: for i in 1..lines.len() {
        let n = i.min(lines.len() - i);

        let (left, right) = &lines[i - n..i + n].split_at(n);

        let mut one_off_count = 0;
        for (a, b) in left.iter().zip(right.iter().rev()) {
            let c = a ^ b;
            let is_one_off = c != 0 && c & (c - 1) == 0;

            one_off_count += is_one_off as u32;

            if !(a == b || is_one_off) {
                continue 'search;
            }
        }

        if one_off_count == 1 {
            return i as u32;
        }
    }

    0
}

pub fn part2(input: &str) -> u32 {
    let patterns = input.split("\n\n");

    patterns
        .map(parse_pattern)
        .map(|bitmap| {
            let a = count_smudged_reflections(&bitmap.cols);
            let b = count_smudged_reflections(&bitmap.rows);
            a + 100 * b
        })
        .sum::<u32>()
}
