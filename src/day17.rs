#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Crucible {
    pos: Position,
    facing: Direction,
    stability: u8,
}

impl Crucible {
    fn as_idx(&self, width: usize, stability_count: usize) -> usize {
        use Direction::*;

        (self.pos.as_idx(width) * stability_count + self.stability as usize) * 4
            + match self.facing {
                Up => 0,
                Right => 1,
                Down => 2,
                Left => 3,
            }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn is_in_bounds(&self, width: usize, height: usize) -> bool {
        (0..width).contains(&(self.x as usize)) && (0..height).contains(&(self.y as usize))
    }

    fn as_idx(&self, width: usize) -> usize {
        self.x as usize + self.y as usize * width
    }
}

fn neighbors(c: Crucible, width: usize, height: usize) -> [Crucible; 3] {
    use Direction::*;

    let p = c.pos;

    let mut res = [Crucible {
        pos: Position { x: -1, y: -1 },
        facing: Up,
        stability: 0,
    }; 3];
    let mut i = 0usize;

    if c.stability != 0 {
        let pos = match c.facing {
            Up => Position { y: p.y - 1, ..p },
            Right => Position { x: p.x + 1, ..p },
            Down => Position { y: p.y + 1, ..p },
            Left => Position { x: p.x - 1, ..p },
        };

        res[i] = Crucible {
            pos,
            stability: c.stability - 1,
            ..c
        };

        i += 1;
    }

    if matches!(c.facing, Direction::Down | Direction::Up) {
        res[i] = Crucible {
            pos: Position { x: p.x - 1, y: p.y },
            facing: Direction::Left,
            stability: 2,
        };
        i += 1;

        res[i] = Crucible {
            pos: Position { x: p.x + 1, y: p.y },
            facing: Direction::Right,
            stability: 2,
        };
        i += 1;
    } else {
        res[i] = Crucible {
            pos: Position { x: p.x, y: p.y - 1 },
            facing: Direction::Up,
            stability: 2,
        };
        i += 1;

        res[i] = Crucible {
            pos: Position { x: p.x, y: p.y + 1 },
            facing: Direction::Down,
            stability: 2,
        };
        i += 1;
    }

    res
}

fn neighbors_ultra(c: Crucible, width: usize, height: usize) -> [Crucible; 3] {
    use Direction::*;

    let p = c.pos;

    let mut res = [Crucible {
        pos: Position { x: -1, y: -1 },
        facing: Up,
        stability: 0,
    }; 3];

    let mut i = 0usize;

    if c.stability != 0 {
        let pos = match c.facing {
            Up => Position { y: p.y - 1, ..p },
            Right => Position { x: p.x + 1, ..p },
            Down => Position { y: p.y + 1, ..p },
            Left => Position { x: p.x - 1, ..p },
        };

        res[i] = Crucible {
            pos,
            stability: c.stability - 1,
            ..c
        };

        i += 1;
    }

    if c.stability > 6 {
        return res;
    }

    if matches!(c.facing, Direction::Down | Direction::Up) {
        res[i] = Crucible {
            pos: Position { x: p.x - 1, y: p.y },
            facing: Direction::Left,
            stability: 9,
        };
        i += 1;

        res[i] = Crucible {
            pos: Position { x: p.x + 1, y: p.y },
            facing: Direction::Right,
            stability: 9,
        };
        i += 1;
    } else {
        res[i] = Crucible {
            pos: Position { x: p.x, y: p.y - 1 },
            facing: Direction::Up,
            stability: 9,
        };
        i += 1;

        res[i] = Crucible {
            pos: Position { x: p.x, y: p.y + 1 },
            facing: Direction::Down,
            stability: 9,
        };
        i += 1;
    }

    res
}

fn manhattan(a: Position, b: Position) -> u32 {
    a.x.abs_diff(b.x) as u32 + a.y.abs_diff(b.y) as u32
}

fn heuristic(p: Position, goal: Position) -> u32 {
    manhattan(p, goal)
}

fn dijkstra<F: Fn(Crucible, usize, usize) -> [Crucible; 3], G: Fn(Crucible) -> bool>(
    heatmap: &[u8],
    width: usize,
    height: usize,
    neighbor: F,
    goal: G,
    start: &[Crucible],
    stability_count: usize,
) -> u32 {
    let state_count = width * height * stability_count * 4;
    let mut frontier = Vec::new();
    frontier.extend(start);

    let mut score = vec![u32::MAX; state_count];
    for s in start {
        score[s.as_idx(width, stability_count)] = 0;
    }

    while !frontier.is_empty() {
        let current: Crucible = frontier.remove(0);
        if goal(current) {
            return score[current.as_idx(width, stability_count)];
        }

        for n in neighbor(current, width, height) {
            if !n.pos.is_in_bounds(width, height) {
                continue;
            }

            let nidx = n.as_idx(width, stability_count);
            let frontier_idx = frontier.iter().position(|x| *x == n);
            let is_in_frontier = frontier_idx.is_some();
            let heatmap_idx = n.pos.as_idx(width);
            let new_score =
                score[current.as_idx(width, stability_count)] + heatmap[heatmap_idx] as u32;

            if score[nidx] == u32::MAX && !is_in_frontier {
                score[nidx] = new_score;

                if let Err(i) | Ok(i) = frontier
                    .binary_search_by(|c| score[c.as_idx(width, stability_count)].cmp(&score[nidx]))
                {
                    frontier.insert(i, n);
                }
            } else {
                if is_in_frontier && new_score < score[nidx] {
                    let Some(frontier_idx) = frontier_idx else {
                        unreachable!()
                    };

                    frontier.remove(frontier_idx);
                    score[nidx] = new_score;

                    if let Err(i) | Ok(i) = frontier.binary_search_by(|c| {
                        score[c.as_idx(width, stability_count)].cmp(&score[nidx])
                    }) {
                        frontier.insert(i, n);
                    }
                }
            }
        }
    }

    panic!()
}

/*
fn astar(heatmap: &[u8], width: usize, height: usize) -> u32 {
    // TODO the crucible can start by going down OR to the right
    let start = Crucible {
        pos: Position { x: 0, y: 0 },
        facing: Direction::Down,
        stability: 3,
    };

    let goal = Position {
        x: width as isize - 1,
        y: height as isize - 1,
    };

    let state_count = width * height * 4 * 4;

    let mut frontier = vec![start];

    let mut gscore = vec![u32::MAX; state_count];
    gscore[start.as_idx(width)] = 0;

    let mut fscore = vec![u32::MAX; state_count];
    fscore[start.as_idx(width)] = heuristic(start.pos, goal);

    while !frontier.is_empty() {
        let current = frontier.remove(0);

        if current.pos == goal {
            return gscore[current.as_idx(width)];
        }

        for n in neighbors(current, width, height).into_iter() {
            if !n.pos.is_in_bounds(width, height) {
                continue;
            }

            let heatmap_idx = n.pos.as_idx(width);
            let nidx = n.as_idx(width);
            let ngscore = gscore[current.as_idx(width)] + heatmap[heatmap_idx] as u32;
            if ngscore < gscore[nidx] {
                gscore[nidx] = ngscore;
                fscore[nidx] = ngscore + heuristic(n.pos, goal);

                if let Err(i) | Ok(i) =
                    frontier.binary_search_by(|c| fscore[c.as_idx(width)].cmp(&fscore[nidx]))
                {
                    frontier.insert(i, n);
                }
            }
        }
    }

    panic!()
}
*/

fn parse_heatmap(input: &str) -> (usize, usize, Vec<u8>) {
    let mut heatmap = Vec::new();
    let mut h = 0;
    for (i, line) in input.lines().enumerate() {
        heatmap.extend(line.as_bytes());
        h = i;
    }

    for p in heatmap.iter_mut() {
        *p -= b'0';
    }

    let height = h + 1;
    let width = heatmap.len() / height;

    (width, height, heatmap)
}

pub fn part1(input: &str) -> u32 {
    let (width, height, heatmap) = parse_heatmap(input);
    let start = Crucible {
        pos: Position { x: 0, y: 0 },
        facing: Direction::Down,
        stability: 3,
    };

    let goal = Position {
        x: width as isize - 1,
        y: height as isize - 1,
    };

    dijkstra(&heatmap, width, height, neighbors, |c| c.pos == goal,&[start], 3)
}

pub fn part2(input: &str) -> u32 {
    let (width, height, heatmap) = parse_heatmap(input);
    let start = [
        Crucible {
            pos: Position { x: 0, y: 0 },
            facing: Direction::Down,
            stability: 10,
        },
        Crucible {
            pos: Position { x: 0, y: 0 },
            facing: Direction::Right,
            stability: 10,
        },
    ];

    let goal_pos = Position {
        x: width as isize - 1,
        y: height as isize - 1,
    };

    dijkstra(&heatmap, width, height, neighbors_ultra, |c| c.pos == goal_pos && c.stability <= 6, &start, 10)
}
