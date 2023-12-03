const INPUT: &str = include_str!("../../input/day_03.txt");

fn part_1(input: &str) {
    let mut symbols = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let line_symbols = line.as_bytes().iter().enumerate().filter_map(|(j, &b)| {
            if !b.is_ascii_digit() && b != b'.' {
                Some((i, j))
            } else {
                None
            }
        });

        symbols.extend(line_symbols);
    }

    println!("{symbols:?}");

    todo!()
}

fn part_2(input: &str) {
    todo!()
}

fn main() {
    part_1(INPUT);
    part_2(INPUT);
}
