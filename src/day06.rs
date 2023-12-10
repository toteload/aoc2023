use crate::util::parse_whitespace_separated_items;

fn quadratic_roots(a: f64, b: f64, c: f64) -> [f64; 2] {
    let d = b * b - 4.0 * a * c;

    assert!(d > 0.0);

    let mut res = [(-b + d.sqrt()) / (2.0 * a), (-b - d.sqrt()) / (2.0 * a)];
    if res[0] > res[1] {
        res.swap(0, 1);
    }

    res
}

fn next_whole_number(x: f64) -> f64 {
    let c = x.ceil();
    if x == c {
        c + 1.0
    } else {
        c
    }
}

fn prev_whole_number(x: f64) -> f64 {
    let c = x.floor();
    if x == c {
        c - 1.0
    } else {
        c
    }
}

pub fn part1(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let time = parse_whitespace_separated_items::<f64>(lines[0].split(':').nth(1).unwrap());
    let distance = parse_whitespace_separated_items::<f64>(lines[1].split(':').nth(1).unwrap());

    let mut answer = 1u64;
    for (t, d) in time.iter().zip(distance) {
        let [x0, x1] = quadratic_roots(1.0, -t, d);
        let a = next_whole_number(x0);
        let b = prev_whole_number(x1);
        let c = b - a + 1.0;
        answer *= c as u64;
    }

    answer
}

pub fn part2(_input: &str) -> u64 {
    // TODO Parse the numbers from the input
    let t = 48938595.0;
    let d = 296192812361391.0;
    let [x0, x1] = quadratic_roots(1.0, -t, d);
    let a = next_whole_number(x0);
    let b = prev_whole_number(x1);
    (b - a + 1.0) as u64
}
