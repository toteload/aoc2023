fn manhattan_distance_u32(x0: u32, y0: u32, x1: u32, y1: u32) -> u32 {
    x0.abs_diff(x1) + y0.abs_diff(y1)
}

fn minmax_u32(a: u32, b: u32) -> [u32; 2] {
    [a.min(b), a.max(b)]
}

pub fn part1(input: &str) -> u32 {
    let image = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let empty_rows = image
        .iter()
        .enumerate()
        .filter_map(|(i, row)| row.iter().all(|c| *c == b'.').then_some(i as u32))
        .collect::<Vec<_>>();

    let mut empty_cols = Vec::new();
    'search: for i in 0..image[0].len() {
        for row in &image {
            if row[i] != b'.' {
                continue 'search;
            }
        }

        empty_cols.push(i as u32);
    }

    let mut galaxies = Vec::new();

    for (y, row) in image.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == b'#' {
                galaxies.push((x as u32, y as u32));
            }
        }
    }

    let mut answer = 0;
    for i in 0..galaxies.len() - 1 {
        let (x0, y0) = galaxies[i];

        for (x1, y1) in &galaxies[i..] {
            let [x0, x1] = minmax_u32(x0, *x1);
            let [y0, y1] = minmax_u32(y0, *y1);

            let col_count = empty_cols.iter().filter(|&&x| x0 < x && x < x1).count() as u32;
            let row_count = empty_rows.iter().filter(|&&y| y0 < y && y < y1).count() as u32;

            answer += manhattan_distance_u32(x0, y0, x1, y1) + col_count + row_count;
        }
    }

    answer
}

fn manhattan_distance_u64(x0: u64, y0: u64, x1: u64, y1: u64) -> u64 {
    x0.abs_diff(x1) + y0.abs_diff(y1)
}

fn minmax_u64(a: u64, b: u64) -> [u64; 2] {
    [a.min(b), a.max(b)]
}

pub fn part2(input: &str) -> u64 {
    let image = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let empty_rows = image
        .iter()
        .enumerate()
        .filter_map(|(i, row)| row.iter().all(|c| *c == b'.').then_some(i as u64))
        .collect::<Vec<_>>();

    let mut empty_cols = Vec::new();
    'search: for i in 0..image[0].len() {
        for row in &image {
            if row[i] != b'.' {
                continue 'search;
            }
        }

        empty_cols.push(i as u64);
    }

    let mut galaxies = Vec::new();

    for (y, row) in image.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == b'#' {
                galaxies.push((x as u64, y as u64));
            }
        }
    }

    let mut answer = 0;
    for i in 0..galaxies.len() - 1 {
        let (x0, y0) = galaxies[i];

        for (x1, y1) in &galaxies[i..] {
            let [x0, x1] = minmax_u64(x0, *x1);
            let [y0, y1] = minmax_u64(y0, *y1);

            let col_count = empty_cols.iter().filter(|&&x| x0 < x && x < x1).count() as u64;
            let row_count = empty_rows.iter().filter(|&&y| y0 < y && y < y1).count() as u64;

            answer +=
                manhattan_distance_u64(x0, y0, x1, y1) + col_count * 999_999 + row_count * 999_999;
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(include_str!("../input/day_11.txt")), 9648398);
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::part2(include_str!("../input/day_11.txt")),
            618800410814
        );
    }
}
