struct Brick {
    pos: [u32;3],
    size: [u32; 3],
}

fn parse_point(s: &str) -> [u32; 3] {
    let mut numbers = s.split(',').map(|x| x.parse::<u32>().unwrap());
    [numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()]
}

fn parse_brick(line: &str) -> Brick {
    let points = line.split('~').map(parse_point);
    let a = points.next().unwrap(); 
    let b = points.next().unwrap();

    todo!()
}

pub fn part1(input: &str) -> u32 {
    let mut bricks = input.lines().map(parse_brick).collect::<Vec<_>>();
    bricks.sort_unstable_by_key(|b| b.pos[2]);
}

pub fn part2(input: &str) -> u32 {
    todo!()
}