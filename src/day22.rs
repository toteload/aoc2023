#[derive(Copy, Clone, PartialEq, Eq)]
struct Brick {
    pos: [u32; 3],
    size: [u32; 3],
}

impl Brick {
    fn projection(&self) -> Projector {
        Projector {
            brick: *self,
            i: 0,
            dim: match self.size {
                [_, 0, 0] => 0,
                [0, _, 0] => 1,
                [0, 0, _] => 2,
                _ => unreachable!(),
            },
        }
    }
}

struct Projector {
    brick: Brick,
    i: usize,
    dim: usize,
}

impl Iterator for Projector {
    type Item = (usize, usize, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.brick.size[self.dim] as usize {
            return None;
        }

        let x = self.brick.pos[0] as usize + if self.dim == 0 { self.i } else { 0 };
        let y = self.brick.pos[1] as usize + if self.dim == 1 { self.i } else { 0 };

        self.i += 1;

        let z = {
            if self.dim == 2 {
                self.i = self.brick.size[2] as usize;
                self.brick.size[2]
            } else {
                1
            }
        };

        Some((x, y, z))
    }
}

fn parse_point(s: &str) -> [u32; 3] {
    let mut numbers = s.split(',').map(|x| x.parse::<u32>().unwrap());
    [
        numbers.next().unwrap(),
        numbers.next().unwrap(),
        numbers.next().unwrap(),
    ]
}

fn parse_brick(line: &str) -> Brick {
    let mut points = line.split('~').map(parse_point);

    let a = points.next().unwrap();
    let b = points.next().unwrap();

    let mut pos = [0; 3];
    let mut size = [0; 3];
    for i in 0..3 {
        pos[i] = a[i].min(b[i]);
        size[i] = a[i].abs_diff(b[i]);
    }

    let i = size.iter().position(|x| *x != 0).unwrap_or(2);
    size[i] += 1;

    Brick { pos, size }
}

pub fn part1(input: &str) -> u32 {
    let mut bricks = input.lines().map(parse_brick).collect::<Vec<_>>();
    bricks.sort_unstable_by_key(|b| b.pos[2]);

    let (width, height) = {
        let (min, max) = bricks
            .iter()
            .map(|b| b.pos[0])
            .fold((0, 0), |(min, max), x| (min.min(x), max.max(x)));
        let width = max - min + 1;

        let (min, max) = bricks
            .iter()
            .map(|b| b.pos[1])
            .fold((0, 0), |(min, max), y| (min.min(y), max.max(y)));
        let height = max - min + 1;

        (width as usize, height as usize)
    };

    let mut safe_to_remove = vec![true; bricks.len()];
    const NIL: usize = usize::MAX;
    let mut depths = vec![(0, NIL); width * height];

    for (brick_idx, brick) in bricks.iter().enumerate() {
        let mut single_support = true;
        let mut last_support = (0, NIL);
        for (x, y, _) in brick.projection() {
            let (h, i) = depths[x + y * width];

            if h < last_support.0 {
                continue;
            }

            if h > last_support.0 {
                last_support = (h, i);
                single_support = true;
                continue;
            }

            if i != last_support.1 {
                single_support = false;
            }
        }

        let (h, i) = last_support;

        for (x, y, z) in brick.projection() {
            depths[x + y * width] = (h + z, brick_idx);
        }

        if single_support && i != NIL {
            safe_to_remove[i] = false;
        }
    }

    let mut count = 0;
    for safe in safe_to_remove {
        if safe {
            count += 1;
        }
    }

    count
}

fn count_falling_bricks(removal: usize, supports: &[&[usize]]) -> u32 {
    let mut infected = vec![false; supports.len()];
    infected[removal] = true;

    for (i, parents) in supports.iter().enumerate().skip(removal+1) {
        if !parents.is_empty() && parents.iter().all(|&j| infected[j]) {
            infected[i] = true;
        }
    }

    let mut count = 0;
    for x in infected {
        if x {
            count += 1;
        }
    }

    count - 1
}

pub fn part2(input: &str) -> u64 {
    let mut bricks = input.lines().map(parse_brick).collect::<Vec<_>>();
    bricks.sort_unstable_by_key(|b| b.pos[2]);

    let (width, height) = {
        let (min, max) = bricks
            .iter()
            .map(|b| b.pos[0])
            .fold((0, 0), |(min, max), x| (min.min(x), max.max(x)));
        let width = max - min + 1;

        let (min, max) = bricks
            .iter()
            .map(|b| b.pos[1])
            .fold((0, 0), |(min, max), y| (min.min(y), max.max(y)));
        let height = max - min + 1;

        (width as usize, height as usize)
    };

    const NIL: usize = usize::MAX;
    let mut depths = vec![(0, NIL); width * height];
    let mut supports = Vec::new();
    let mut support_ranges = Vec::new();

    for (brick_idx, brick) in bricks.iter().enumerate() {
        let start = supports.len();

        let mut last_height = 0;
        for (x, y, _) in brick.projection() {
            let (h, i) = depths[x + y * width];

            if h < last_height {
                continue;
            }

            if h > last_height {
                last_height = h;
                supports.truncate(start);
            }

            if i != NIL {
                supports.push(i);
            }
        }

        let end = supports.len();
        support_ranges.push(start..end);

        for (x, y, z) in brick.projection() {
            depths[x + y * width] = (last_height + z, brick_idx);
        }
    }

    let support_data = supports;
    let supports = support_ranges.into_iter().map(|r| &support_data[r]).collect::<Vec<_>>();

    (0..bricks.len()).map(|i| count_falling_bricks(i, &supports) as u64).sum::<u64>()
}
