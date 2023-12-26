fn parse_line(line: &str) -> ([f64; 3], [f64; 3]) {
    let mut parts = line.split(" @ ");

    let mut ps = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|p| p.trim().parse::<i64>().unwrap() as f64);

    let pos = [ps.next().unwrap(), ps.next().unwrap(), ps.next().unwrap()];

    let mut vs = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|v| v.trim().parse::<i64>().unwrap() as f64);

    let vel = [vs.next().unwrap(), vs.next().unwrap(), vs.next().unwrap()];

    (pos, vel)
}

pub fn part1(input: &str) -> u32 {
    let stones = input.lines().map(parse_line).collect::<Vec<_>>();

    let lo = 200000000000000.0;
    let hi = 400000000000000.0;

    let mut answer = 0;

    for (i, a) in stones[..(stones.len() - 1)].iter().enumerate() {
        for b in &stones[(i + 1)..] {
            let ([ax, ay, _], [avx, avy, _]) = a;
            let ([bx, by, _], [bvx, bvy, _]) = b;

            let s = avx / avy;
            let q = (bx - ax - (s * (by - ay))) / (s * bvy - bvx);

            let s = bvx / bvy;
            let r = (ax - bx - (s * (ay - by))) / (s * avy - avx);

            // q is negative if the intersection is in the past for stone b.
            // if one of them is infinite, then both are infinite (I think maybe).
            if q.is_infinite() || q < 0.0 || r < 0.0 {
                continue;
            }

            let x = bx + bvx * q;
            let y = by + bvy * q;

            if x < lo || x > hi || y < lo || y > hi {
                continue;
            }

            answer += 1;
        }
    }

    answer
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn part2(input: &str) -> u32 {
    let stones = input.lines().map(parse_line).collect::<Vec<_>>();

    for i in 0..stones.len() - 1 {
        for j in i..stones.len() {
            let ([ax, ay, az], [avx, avy, avz]) = stones[i];
            let ([bx, by, bz], [bvx, bvy, bvz]) = stones[j];

            let p0 = [bx - (ax + avx), by - (ay + avy), bz - (az + avz)];
            let p1 = [bvx, bvy, bvz];

            let n = [
                p0[1] * p1[2] - p0[2] * p1[1],
                p0[2] * p0[0] - p0[0] * p1[2],
                p0[0] * p0[1] - p0[1] * p1[0],
            ];

            let c = stones[(j + 1) % stones.len()];
            let d = stones[(j + 2) % stones.len()];

            let ([cx, cy, cz], [cvx, cvy, cvz]) = c;
            let ([dx, dy, dz], [dvx, dvy, dvz]) = d;

            let ipc = dot(n, [ax - cx, ay - cy, az - cz]) / dot(c.1, n);
            let ipd = dot(n, [ax - dx, ay - dy, az - dz]) / dot(d.1, n);

            // Calculate the intersection points of c and d and the plane
            let ic = [cx + ipc * cvx, cy + ipc * cvy, cz + ipc * cvz];

            let id = [dx + ipd * dvx, dy + ipd * dvy, dz + ipd * dvz];

            let ac = [ic[0] - ax, ic[1] - ay, ic[2] - az];

            let dc = [ic[0] - id[0], ic[1] - id[1], ic[2] - id[2]];

            if ac.iter().zip(dc.iter()).all(|(a, b)| a == b)
                || ac.iter().zip(dc.iter()).all(|(a, b)| -a == *b)
            {
                println!("MATCH? {i} {j}, {ipc} {ipd} {ic:?} {id:?}");
            }
        }
    }

    todo!()
}
