use std::str::FromStr;

pub fn parse_whitespace_separated_items<T: FromStr>(s: &str) -> Vec<T> {
    s.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<T>().ok().unwrap())
        .collect()
}

// Greatest Common Divisor
pub fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

// Least Common Multiple
pub fn lcm(a: i64, b: i64) -> i64 {
    let t = b / gcd(a, b);
    a * t
}

// Give `a` and `b` calculate Bezout's coefficients `x` and `y` and the greatest common divisor
// (gcd). Bezout's coefficients are used in Bezout's identity: ax + by = gcd(a, b).
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut or, mut r) = (a, b);
    let (mut os, mut s) = (1, 0);
    let (mut ot, mut t) = (0, 1);

    while r != 0 {
        let q = or / r;
        (or, r) = (r, or - q * r);
        (os, s) = (s, os - q * s);
        (ot, t) = (t, ot - q * t);
    }

    (os, ot, or)
}

// Chinese Remainder Theorem
// let test = vec![ (0, 3), (3, 4), (4, 5) ];
// let (a, n) = test.iter().copied().reduce(|(a0, n0), (a1, n1)| {
//     let (x, y, _) = extended_gcd(n0, n1);
//     let z = (a0 * y * n1 + a1 * x * n0) % (n0 * n1);
//     (if z < 0 { z + (n0 + n1) } else { z }, n0 * n1)
// }).unwrap();
