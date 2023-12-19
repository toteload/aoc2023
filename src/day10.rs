const HEIGHT: usize = 140;
const WIDTH: usize = 140;

const LEFT: u8 = 0b0001;
const DOWN: u8 = 0b0010;
const RIGHT: u8 = 0b0100;
const UP: u8 = 0b1000;

fn parse_tile(tile: u8) -> u8 {
    match tile {
        b'|' => UP | DOWN,
        b'-' => LEFT | RIGHT,
        b'L' => UP | RIGHT,
        b'J' => UP | LEFT,
        b'7' => LEFT | DOWN,
        b'F' => RIGHT | DOWN,
        _ => 0,
    }
}

fn opposite_direction(direction: u8) -> u8 {
    match direction {
        LEFT => RIGHT,
        RIGHT => LEFT,
        DOWN => UP,
        UP => DOWN,
        _ => unreachable!(),
    }
}

fn determine_start_directions(tiles: &[u8], start: usize) -> (u8, u8) {
    let mut i = 0;
    let mut res = [0; 2];

    if start / WIDTH > 0 {
        let up = start - WIDTH;
        if tiles[up] & DOWN != 0 {
            res[i] = UP;
            i += 1;
        }
    }

    if start / WIDTH < HEIGHT - 1 {
        let down = start + WIDTH;
        if tiles[down] & UP != 0 {
            res[i] = DOWN;
            i += 1;
        }
    }

    if start % WIDTH > 0 {
        let left = start - 1;
        if tiles[left] & RIGHT != 0 {
            res[i] = LEFT;
            i += 1;
        }
    }

    if start % WIDTH < WIDTH - 1 {
        let right = start + 1;
        if tiles[right] & LEFT != 0 {
            res[i] = RIGHT;
            i += 1;
        }
    }

    debug_assert_eq!(i, 2);

    (res[0], res[1])
}

fn step(tiles: &[u8], position: usize, direction_to: u8) -> (usize, u8, u8) {
    let offset: isize = match direction_to {
        LEFT => -1,
        DOWN => WIDTH as isize,
        RIGHT => 1,
        UP => -(WIDTH as isize),
        _ => unreachable!(),
    };

    let position = position.wrapping_add_signed(offset);
    let direction_from = opposite_direction(direction_to);
    let direction_to = tiles[position] & !direction_from;

    (position, direction_to, direction_from)
}

pub fn part1(input: &str) -> u32 {
    let mut tiles = Vec::with_capacity(WIDTH * HEIGHT);

    let lines = input.lines();
    for line in lines.clone() {
        let row = line.as_bytes().iter().copied().map(parse_tile);
        tiles.extend(row);
    }

    debug_assert_eq!(tiles.len(), WIDTH * HEIGHT);

    let start = 'search: {
        for (y, line) in lines.enumerate() {
            if let Some(x) = line.bytes().position(|b| b == b'S') {
                break 'search y * WIDTH + x;
            }
        }

        unreachable!();
    };

    let (da, db) = determine_start_directions(&tiles, start);

    let mut distance = 0;
    let (mut a, mut da) = (start, da);
    let (mut b, mut db) = (start, db);

    // TODO You probably don't have to step from both directions. You might be able to complete a
    // loop going in one direction and then take half the length.

    loop {
        (a, da, _) = step(&tiles, a, da);

        if a == b {
            // This means that a and b will pass each other at this step.
            break;
        }

        distance += 1;

        (b, db, _) = step(&tiles, b, db);

        if a == b {
            break;
        }
    }

    distance
}

pub fn part2(input: &str) -> u32 {
    let mut tiles = Vec::with_capacity(WIDTH * HEIGHT);

    let lines = input.lines();
    for line in lines.clone() {
        let row = line.as_bytes().iter().copied().map(parse_tile);
        tiles.extend(row);
    }

    debug_assert_eq!(tiles.len(), WIDTH * HEIGHT);

    let start = 'search: {
        for (y, line) in lines.enumerate() {
            if let Some(x) = line.bytes().position(|b| b == b'S') {
                break 'search y * WIDTH + x;
            }
        }

        unreachable!();
    };

    let mut outline_tiles = vec![0i8; WIDTH * HEIGHT];

    let ds = determine_start_directions(&tiles, start);

    let (mut p, mut dir_to, mut dir_from) = (start, ds.0, opposite_direction(ds.0));
    tiles[start] = ds.0 | ds.1;

    loop {
        outline_tiles[p] = match (dir_to, dir_from) {
            (UP, _) | (_, DOWN) => 1,
            (DOWN, _) | (_, UP) => -1,
            _ => 2,
        };

        (p, dir_to, dir_from) = step(&tiles, p, dir_to);

        if p == start {
            break;
        }
    }

    let mut acc = 0;
    for row in outline_tiles.chunks_exact(WIDTH) {
        let mut winding: i8 = 0;
        let mut last = 0;

        for tile in row {
            if *tile == 0 {
                if winding != 0 {
                    acc += 1;
                }
            } else if *tile != 2 && *tile != last {
                winding += tile;
                last = *tile;
            }
        }
    }

    acc
}
