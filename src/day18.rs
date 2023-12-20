use core::ops::Range;

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse_line_part1(line: &str) -> (Direction, isize) {
    use Direction::*;

    let bs = line.as_bytes();
    let dir = match bs[0] {
        b'U' => Up,
        b'R' => Right,
        b'D' => Down,
        b'L' => Left,
        _ => unreachable!(),
    };

    let d = line
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse::<isize>()
        .unwrap();

    (dir, d as isize)
}

pub fn part1(input: &str) -> u32 {
    let mut corners = vec![(0isize, 0isize)];

    for (dir, d) in input.lines().map(parse_line_part1) {
        let (x, y) = *corners.last().unwrap();
        use Direction::*;
        let p = match dir {
            Up => (x, y - d),
            Right => (x + d, y),
            Down => (x, y + d),
            Left => (x - d, y),
        };
        corners.push(p);
    }

    let mut minx = 0;
    let mut maxx = 0;
    let mut miny = 0;
    let mut maxy = 0;

    for (x, y) in &corners {
        minx = minx.min(*x);
        maxx = maxx.max(*x);
        miny = miny.min(*y);
        maxy = maxy.max(*y);
    }

    let width = 1 + (maxx - minx) as usize;
    let height = 1 + (maxy - miny) as usize;

    let mut bitmap = vec![0i8; width * height];

    for ((dir, d), (x, y)) in input.lines().map(parse_line_part1).zip(corners.iter()) {
        if dir == Direction::Up {
            for dy in 0..=d {
                let i = (x - minx) as usize + (y - miny - dy) as usize * width;
                bitmap[i] = 1;
            }
        }

        if dir == Direction::Down {
            for dy in 0..=d {
                let i = (x - minx) as usize + (y - miny + dy) as usize * width;
                bitmap[i] = -1;
            }
        }

        if matches!(dir, Direction::Left | Direction::Right) {
            let sign = if dir == Direction::Left { -1 } else { 1 };
            for dx in 1..d {
                let i = (x - minx + sign * dx) as usize + (y - miny) as usize * width;
                bitmap[i] = 2;
            }
        }
    }

    let mut answer = 0;
    for row in bitmap.as_slice().chunks_exact(width) {
        let mut winding = 0;
        let mut last_winding = 0;
        for p in row {
            if *p != 0 || winding != 0 {
                answer += 1;
            }

            if matches!(*p, -1 | 1) && *p != last_winding {
                winding += *p;
                last_winding = *p;
            }
        }
    }

    answer
}

fn parse_line_part2(line: &str) -> (Direction, isize) {
    let x = line.split_ascii_whitespace().nth(2).unwrap();
    let d = isize::from_str_radix(&x[2..7], 16).unwrap();

    use Direction::*;
    let dir = match &x[7..8] {
        "0" => Right,
        "1" => Down,
        "2" => Left,
        "3" => Up,
        _ => unreachable!(),
    };

    (dir, d)
}

fn overlap<Idx: Ord + Copy>(a: &Range<Idx>, b: &Range<Idx>) -> Range<Idx> {
    if b.start >= a.end || a.start >= b.end {
        a.start..a.start
    } else {
        (a.start.max(b.start))..(a.end.min(b.end))
    }
}

pub fn part2(input: &str) -> usize {
    let instructions = input.lines().map(parse_line_part2);

    let mut ys = Vec::new();
    let mut segments = Vec::new();
    let mut at = (0, 0);

    for (dir, distance) in instructions {
        use Direction::*;

        let end = {
            let (x, y) = at;

            match dir {
                Right => (x + distance, y),
                Down => (x, y + distance),
                Left => (x - distance, y),
                Up => (x, y - distance),
            }
        };

        let y = end.1.max(at.1);

        if let Err(idx) = ys.binary_search(&y) {
            ys.insert(idx, y);
        }

        if matches!(dir, Down | Up) {
            segments.push((at, end));
        }

        at = end;
    }

    let mut answer = 0;

    let mut xxs = Vec::new();

    for hs in ys.windows(2) {
        let (h, y) = {
            let &[a, b] = hs else { unreachable!() };
            let h = b - a + 1;
            (h as usize, a)
        };

        let mut xs = Vec::new();

        for ((x, y0), (_, y1)) in &segments {
            let ya = *y0.min(y1);
            let yb = *y0.max(y1);
            if (ya..yb).contains(&y) {
                if let Err(idx) = xs.binary_search(&x) {
                    xs.insert(idx, x);
                }
            }
        }

        let scanned = xs
            .as_slice()
            .chunks_exact(2)
            .map(|x| (x[1] - x[0] + 1) as usize)
            .sum::<usize>();
        xxs.push(xs);

        answer += scanned * h;
    }

    for window in xxs.as_slice().windows(2) {
        let [a, b] = window else { unreachable!() };

        for x in a.as_slice().chunks_exact(2).map(|x| (*x[0]..(x[1] + 1))) {
            for y in b.as_slice().chunks_exact(2).map(|x| (*x[0]..(x[1] + 1))) {
                answer -= overlap(&x, &y).len();
            }
        }
    }

    answer
}
