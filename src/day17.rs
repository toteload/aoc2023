enum Direction { UP, RIGHT, DOWN, LEFT }

struct Crucible {
    pos: Position,
    facing: Direction,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn as_idx(&self, width: usize) -> usize {
        self.x + self.y * width
    }
}

fn neighbors(p: Position, width: usize, height: usize) -> [Option<Position>; 9] {
    // This is wrong. The crucible can not reverse.
    todo!();
    
    let mut res = [None; 9];
    let mut i = 0usize;

    for dx in 1..4 {
        if dx > p.x {
            break;
        }

        res[i] = Some(Position {
            x: p.x - dx,
            y: p.y,
        });
        i += 1;
    }

    for dx in 1..4 {
        if p.x + dx >= width {
            break;
        }

        res[i] = Some(Position {
            x: p.x - dx,
            y: p.y,
        });
        i += 1;
    }

    for dy in 1..4 {
        if dy > p.y {
            break;
        }

        res[i] = Some(Position {
            x: p.x,
            y: p.y - dy,
        });
        i += 1;
    }

    for dy in 1..4 {
        if p.y + dy >= height {
            break;
        }

        res[i] = Some(Position {
            x: p.x,
            y: p.y - dy,
        });
        i += 1;
    }

    res
}

fn manhattan(a: Position, b: Position) -> u32 {
    a.x.abs_diff(b.x) as u32 + a.y.abs_diff(b.y) as u32
}

fn heuristic(p: Position, goal: Position) -> u32 {
    manhattan(p, goal) * 5
}

fn astar(heatmap: &[u8], width: usize, height: usize) -> u32 {
    let start = Position { x: 0, y: 0 };
    let goal = Position { x: width - 1, y: height - 1 };

    let mut frontier = vec![start];

    let mut gscore = vec![u32::MAX; width * height];
    gscore[start.as_idx(width)] = 0;

    let mut fscore = vec![u32::MAX; width * height];
    fscore[start.as_idx(width)] = heuristic(start, goal);

    while !frontier.is_empty() {
        let current = frontier.remove(0);

        if current == goal {
            return gscore[current.as_idx(width)];
        }

        for n in neighbors(current, width, height).into_iter() {
            let Some(n) = n else { break; };
            let nidx = n.as_idx(width);
            let ngscore = gscore[current.as_idx(width)] + heatmap[nidx] as u32;
            if ngscore < gscore[nidx] {
                gscore[nidx] = ngscore;
                fscore[nidx] = ngscore + heuristic(n, goal);

                if let Err(i) = frontier.binary_search_by(|p| fscore[p.as_idx(width)].cmp(&fscore[nidx])) {
                    frontier.insert(i, n);
                }
            }
        }
    }

    panic!()
}

pub fn part1(input: &str) -> u32 {
    todo!()
}

pub fn part2(input: &str) -> u32 {
    todo!()
}
