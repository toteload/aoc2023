use ahash::{HashSet, HashSetExt};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    const fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Bitmap {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Bitmap {
    fn parse(input: &str) -> Bitmap {
        let lines = input.lines().collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();

        let data = lines
            .into_iter()
            .flat_map(|line| line.as_bytes().iter().copied())
            .collect();

        Bitmap {
            data,
            width,
            height,
        }
    }

    fn get(&self, p: &Point) -> u8 {
        self.data[p.x as usize + p.y as usize * self.width]
    }

    fn is_point_in_bounds(&self, p: &Point) -> bool {
        (0..self.width as isize).contains(&p.x) && (0..self.height as isize).contains(&p.y)
    }
}

const UP: Point = Point::new(0, -1);
const DOWN: Point = Point::new(0, 1);
const LEFT: Point = Point::new(-1, 0);
const RIGHT: Point = Point::new(1, 0);

fn count_energized_tiles(grid: &Bitmap, start: &(Point, Point)) -> u32 {
    let mut visited = HashSet::new();
    let mut rays = vec![*start];

    while let Some(ray) = rays.pop() {
        let (mut p, mut step) = ray;

        while grid.is_point_in_bounds(&p) {
            if !visited.insert((p, step)) {
                break;
            }

            match grid.get(&p) {
                b'-' if step == UP || step == DOWN => {
                    rays.push((p.add(LEFT), LEFT));
                    step = RIGHT;
                }

                b'|' if step == LEFT || step == RIGHT => {
                    rays.push((p.add(DOWN), DOWN));
                    step = UP;
                }
                b'/' => {
                    step = match step {
                        RIGHT => UP,
                        LEFT => DOWN,
                        DOWN => LEFT,
                        UP => RIGHT,
                        _ => unreachable!(),
                    };
                }
                b'\\' => {
                    step = match step {
                        RIGHT => DOWN,
                        LEFT => UP,
                        DOWN => RIGHT,
                        UP => LEFT,
                        _ => unreachable!(),
                    };
                }
                _ => (),
            }

            p = p.add(step);
        }
    }

    let unique_positions = visited.into_iter().map(|(p, _)| p).collect::<HashSet<_>>();

    unique_positions.len() as u32
}

pub fn part1(input: &str) -> u32 {
    let grid = Bitmap::parse(input);
    count_energized_tiles(&grid, &(Point::new(0, 0), RIGHT))
}

pub fn part2(input: &str) -> u32 {
    let grid = Bitmap::parse(input);

    let mut answer = 0;

    for x in 0..grid.width {
        let e = count_energized_tiles(&grid, &(Point::new(x as isize, 0), DOWN));
        answer = answer.max(e);

        let e = count_energized_tiles(
            &grid,
            &(Point::new(x as isize, (grid.height - 1) as isize), UP),
        );
        answer = answer.max(e);
    }

    for y in 0..grid.height {
        let e = count_energized_tiles(&grid, &(Point::new(0, y as isize), RIGHT));
        answer = answer.max(e);

        let e = count_energized_tiles(
            &grid,
            &(Point::new((grid.width - 1) as isize, y as isize), LEFT),
        );
        answer = answer.max(e);
    }

    answer
}
