#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
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

    let d = line.split_ascii_whitespace().nth(1).unwrap().parse::<isize>().unwrap();

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
    todo!()
}

pub fn part2(input: &str) -> u32 {
    todo!()
}
