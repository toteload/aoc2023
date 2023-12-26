struct Map {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Debug)]
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

    fn right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn down(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn up(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn is_in_bounds(&self, width: usize, height: usize) -> bool {
        (0..width as isize).contains(&self.x) && (0..height as isize).contains(&self.y)
    }

    fn as_index(&self, width: usize) -> usize {
        self.x as usize + self.y as usize * width
    }
}

fn parse_map(input: &str) -> Map {
    let mut height = 0;
    let mut tiles = Vec::new();

    for (i, line) in input.lines().enumerate() {
        tiles.extend(line.as_bytes());
        height = i;
    }

    height += 1;
    let width = tiles.len() / height;

    Map {
        tiles,
        width,
        height,
    }
}

struct State {
    p: Position,
    d: u32,
    visited: Vec<bool>,
}

fn search(
    p: Position,
    d: u32,
    visited: Vec<bool>,
    width: usize,
    height: usize,
    tiles: &[u8],
    goal: usize,
) -> u32 {
    let mut stack = vec![State { p, d, visited }];

    let mut answer = 0;

    while let Some(State { p, d, mut visited }) = stack.pop() {
        let idx = p.as_index(width);
        if idx == goal {
            answer = answer.max(d);
            continue;
        }

        visited[idx] = true;

        match tiles[idx] {
            b'v' => stack.push(State {
                p: p.down(),
                d: d + 1,
                visited,
            }),
            b'>' => stack.push(State {
                p: p.right(),
                d: d + 1,
                visited,
            }),
            b'<' => stack.push(State {
                p: p.left(),
                d: d + 1,
                visited,
            }),
            b'^' => stack.push(State {
                p: p.up(),
                d: d + 1,
                visited,
            }),
            b'.' => {
                'blk: {
                    let p = p.up();
                    if !p.is_in_bounds(width, height) {
                        break 'blk;
                    }

                    let idx = p.as_index(width);
                    if tiles[idx] == b'#' || tiles[idx] == b'v' || visited[idx] {
                        break 'blk;
                    }

                    stack.push(State {
                        p,
                        d: d + 1,
                        visited: visited.clone(),
                    });
                }

                'blk: {
                    let p = p.right();
                    if !p.is_in_bounds(width, height) {
                        break 'blk;
                    }

                    let idx = p.as_index(width);
                    if tiles[idx] == b'#' || tiles[idx] == b'<' || visited[idx] {
                        break 'blk;
                    }

                    stack.push(State {
                        p,
                        d: d + 1,
                        visited: visited.clone(),
                    });
                }

                'blk: {
                    let p = p.left();
                    if !p.is_in_bounds(width, height) {
                        break 'blk;
                    }

                    let idx = p.as_index(width);
                    if tiles[idx] == b'#' || tiles[idx] == b'>' || visited[idx] {
                        break 'blk;
                    }

                    stack.push(State {
                        p,
                        d: d + 1,
                        visited: visited.clone(),
                    });
                }

                'blk: {
                    let p = p.down();
                    if !p.is_in_bounds(width, height) {
                        break 'blk;
                    }

                    let idx = p.as_index(width);
                    if tiles[idx] == b'#' || tiles[idx] == b'^' || visited[idx] {
                        break 'blk;
                    }

                    stack.push(State {
                        p,
                        d: d + 1,
                        visited: visited.clone(),
                    });
                }
            }
            _ => unreachable!(),
        }
    }

    answer
}

pub fn part1(input: &str) -> u32 {
    let Map {
        tiles,
        width,
        height,
    } = parse_map(input);

    let visited = vec![false; width * height];
    let goal = (Position {
        x: (width - 2) as isize,
        y: (height - 1) as isize,
    })
    .as_index(width);

    search(
        Position { x: 1, y: 0 },
        0,
        visited,
        width,
        height,
        &tiles,
        goal,
    )
}

fn search2(
    p: Position,
    d: u32,
    visited: Vec<bool>,
    width: usize,
    height: usize,
    tiles: &[u8],
    goal: usize,
) -> u32 {
    let mut stack = vec![State { p, d, visited }];

    let mut answer = 0;

    while let Some(State { p, d, mut visited }) = stack.pop() {
        let idx = p.as_index(width);
        if idx == goal {
            answer = answer.max(d);
            continue;
        }

        visited[idx] = true;

        match tiles[idx] {
            b'v' | b'>' | b'<' | b'^' | b'.' => {
                'blk: {
                    let p = p.up();
                    if !p.is_in_bounds(width, height) {
                        break 'blk;
                    }

                    let idx = p.as_index(width);
                    if tiles[idx] == b'#' || visited[idx] {
                        break 'blk;
                    }

                    stack.push(State {
                        p,
                        d: d + 1,
                        visited: visited.clone(),
                    });
                }

                'blk: {
                    let p = p.right();
                    if !p.is_in_bounds(width, height) {
                        break 'blk;
                    }

                    let idx = p.as_index(width);
                    if tiles[idx] == b'#' || visited[idx] {
                        break 'blk;
                    }

                    stack.push(State {
                        p,
                        d: d + 1,
                        visited: visited.clone(),
                    });
                }

                'blk: {
                    let p = p.left();
                    if !p.is_in_bounds(width, height) {
                        break 'blk;
                    }

                    let idx = p.as_index(width);
                    if tiles[idx] == b'#' || visited[idx] {
                        break 'blk;
                    }

                    stack.push(State {
                        p,
                        d: d + 1,
                        visited: visited.clone(),
                    });
                }

                'blk: {
                    let p = p.down();
                    if !p.is_in_bounds(width, height) {
                        break 'blk;
                    }

                    let idx = p.as_index(width);
                    if tiles[idx] == b'#' || visited[idx] {
                        break 'blk;
                    }

                    stack.push(State {
                        p,
                        d: d + 1,
                        visited: visited.clone(),
                    });
                }
            }
            _ => unreachable!(),
        }
    }

    answer
}

pub fn part2(input: &str) -> u32 {
    let Map {
        tiles,
        width,
        height,
    } = parse_map(input);

    let visited = vec![false; width * height];
    let goal = (Position {
        x: (width - 2) as isize,
        y: (height - 1) as isize,
    })
    .as_index(width);

    search2(
        Position { x: 1, y: 0 },
        0,
        visited,
        width,
        height,
        &tiles,
        goal,
    )
}
