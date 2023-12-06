use std::str::FromStr;

pub fn parse_whitespace_separated_items<T: FromStr>(s: &str) -> Vec<T> {
    s.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<T>().ok().unwrap())
        .collect()
}
