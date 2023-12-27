use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn neighbors(&self) -> [Position; 4] {
        [
            Position {
                x: self.x,
                y: self.y - 1,
            },
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x,
                y: self.y + 1,
            },
            Position {
                x: self.x - 1,
                y: self.y,
            },
        ]
    }

    fn is_valid(&self, width: usize, height: usize) -> bool {
        (0..width as isize).contains(&self.x) && (0..height as isize).contains(&self.y)
    }

    fn as_index(&self, width: usize) -> usize {
        self.x as usize + self.y as usize * width
    }
}

struct Garden {
    plots: Vec<u8>,
    width: usize,
    height: usize,
}

fn parse(input: &str) -> (Position, Garden) {
    let lines = input.lines();
    let mut height = 0;
    let mut plots = Vec::new();

    for (i, line) in lines.enumerate() {
        plots.extend(line.as_bytes());
        height = i;
    }

    height += 1;
    let width = plots.len() / height;
    let mut start = 0;

    for (i, c) in plots.iter_mut().enumerate() {
        if *c == b'S' {
            start = i;
            *c = b'.';
            break;
        }
    }

    let start = Position {
        x: (start % width) as isize,
        y: (start / width) as isize,
    };

    (
        start,
        Garden {
            plots,
            width,
            height,
        },
    )
}

pub fn part1(input: &str) -> u32 {
    let (
        start,
        Garden {
            plots,
            width,
            height,
        },
    ) = parse(input);

    let mut ds = vec![u32::MAX; plots.len()];

    let mut frontier = VecDeque::new();
    frontier.push_back((0, start));

    let max_distance = 64;

    while let Some((d, p)) = frontier.pop_front() {
        if d > max_distance
            || !p.is_valid(width, height)
            || ds[p.as_index(width)] != u32::MAX
            || plots[p.as_index(width)] == b'#'
        {
            continue;
        }

        ds[p.as_index(width)] = d;

        for n in p.neighbors() {
            frontier.push_back((d + 1, n));
        }
    }

    let mut answer = 0;
    for d in ds {
        if d % 2 == 0 {
            answer += 1;
        }
    }

    answer
}

fn count_plots(garden: &Garden, start: Position, max_distance: u32) -> u32 {
    let Garden { plots, width, height } = garden;
    let mut frontier = VecDeque::new();
    frontier.push_back((0, start));

    const NIL: u32 = u32::MAX - 1;
    let mut ds = vec![NIL; plots.len()];

    while let Some((d, p)) = frontier.pop_front() {
        if !p.is_valid(*width, *height) {
            continue;
        }

        let idx = p.as_index(*width);
        if d > max_distance
            || ds[idx] != NIL
            || plots[idx] == b'#'
        {
            continue;
        }

        ds[idx] = d;

        for n in p.neighbors() {
            frontier.push_back((d + 1, n));
        }
    }

    // OPTIMIZE You don't have to actually check if the distance is odd or even. All odd and even
    // distances form a checker board pattern with Manhattan distance, so you only need to know the
    // position and whether it was visited or not.

    let mut answer = 0;
    for d in ds {
        if d % 2 == 1 {
            answer += 1;
        }
    }

    answer
}

fn count_column_plots(garden: &Garden, start: Position, n: usize, max_steps: usize) -> u32 {
    let mut answer = 0;

    answer += count_plots(&garden, start, (n / 2 + max_steps % n) as u32);

    if max_steps % n > n / 2 {
        answer += count_plots(&garden, start, (max_steps % n - n / 2) as u32);
    }

    answer
}

pub fn part2(input: &str) -> u64 {
    let (
        start,
        ref garden @ Garden {
            ref plots,
            width,
            height,
        },
    ) = parse(input);

    debug_assert_eq!(width, height);

    // I assume that the starting position is always the center.

    let n = width; 

    let max_steps = 26501365;
    //let max_steps = 50;

    let full_garden_distance = max_steps / n;
    let full_garden_count = {
        let m = full_garden_distance - 1;

        // OG formula: 4 * ((n * (n + 1)) / 2) + 1
        2 * (m * (m + 1)) + 1
    };

    let full_garden_odd_plot_count = {
        let mut count = 0;

        for c in plots.iter().skip(1).step_by(2) {
            if *c != b'#' {
                count += 1;
            }
        }

        count
    };

    let mut answer = full_garden_count as u64 * full_garden_odd_plot_count;

    let r = (max_steps % n) as u32;

    // Columns
    // -------

    // Top
    let start = Position { x: (n / 2) as isize, y: (n - 1) as isize };
    answer += dbg!(count_column_plots(&garden, start, n, max_steps) as u64);

    // Bottom
    let start = Position { x: (n / 2) as isize, y: 0 };
    answer += count_column_plots(&garden, start, n, max_steps) as u64;

    // Right
    let right = Position { x: 0, y: (n / 2) as isize };
    answer += count_column_plots(&garden, start, n, max_steps) as u64;

    // Left
    let start = Position { x: (n - 1) as isize, y: (n / 2) as isize };
    answer += count_column_plots(&garden, start, n, max_steps) as u64;

    println!("{answer}");

    // Diagonals
    // ---------

    let m = full_garden_distance as u64;
    let s = n as u32;

    // Top right
    answer += dbg!(count_plots(&garden, Position { x: 0, y: (n - 1) as isize }, s + r)) as u64 * m;
    if r > s / 2 {
        answer += dbg!(count_plots(&garden, Position { x: 0, y: (n - 1) as isize }, r - s / 2)) as u64 * (m + 1);
    }

    // Bottom right
    answer += dbg!(count_plots(&garden, Position { x: 0, y: 0 }, s + r)) as u64 * m;
    if r > s / 2 {
        answer += count_plots(&garden, Position { x: 0, y: 0 }, r - s / 2) as u64 * (m + 1);
    }

    // Top left
    answer += dbg!(count_plots(&garden, Position { x: (n - 1) as isize, y: (n - 1) as isize }, s + r)) as u64 * m;
    if r > s / 2 {
        answer += count_plots(&garden, Position { x: (n - 1) as isize, y: (n - 1) as isize }, r - s / 2) as u64 * (m + 1);
    }

    // Bottom left
    answer += dbg!(count_plots(&garden, Position { x: (n - 1) as isize, y: 0 }, s + r)) as u64 * m;
    if r > s / 2 {
        answer += count_plots(&garden, Position { x: (n - 1) as isize, y: 0 }, r - s / 2) as u64 * (m + 1);
    }

    answer
}
