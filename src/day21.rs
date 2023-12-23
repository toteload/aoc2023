use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn neighbors(&self) -> [Position; 4] {
        [
            Position { x: self.x, y: self.y - 1},
            Position { x: self.x + 1, y: self.y},
            Position { x: self.x, y: self.y + 1},
            Position { x: self.x - 1, y: self.y},
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

    (start, Garden { plots, width, height })
}

pub fn part1(input: &str) -> u32 {
    let (start, Garden { plots, width, height }) = parse(input);

    let mut ds = vec![u32::MAX; plots.len()];

    let mut frontier = VecDeque::new();
    frontier.push_back((0, start));

    while let Some((d, p)) = frontier.pop_front() {
        if d == 65 || !p.is_valid(width, height) || ds[p.as_index(width)] != u32::MAX || plots[p.as_index(width)] == b'#' {
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

struct Chunk {
    x: isize,
    y: isize,
    start: Position,
    ds: Vec<u32>,
    frontier: VecDeque<(u32, Position)>,
}

impl Chunk {
    fn new(start: Position, d: u32, garden: &Garden, first_start: Position) -> Chunk {
        todo!()
    }

    fn is_explored(&self) -> bool {
        self.frontier.is_empty()
    }

    fn count_odd_distances(&self) -> u64 {
        todo!()
    }
}

pub fn part2(input: &str) -> u64 {
    let (start, garden) = parse(input);
    let mut chunks = Vec::new();
    chunks.push(Chunk::new(start, 0, &garden, start));

    let mut answer = 0;

    while let Some(chunk) = chunks.pop() {
        
    }

    answer
}
