use crate::util::parse_whitespace_separated_items;

fn calculate_differences(seq: &[i64], buf: &mut Vec<i64>) {
    buf.resize(231, 0);

    buf.as_mut_slice()[..21].copy_from_slice(seq);

    let mut offset = 0;
    for n in (1..21).rev() {
        for i in offset..(offset + n) {
            buf[i + n + 1] = buf[i + 1] - buf[i];
        }

        offset = offset + n + 1;
    }
}

pub fn part1(input: &str) -> i64 {
    let mut buf = Vec::with_capacity(231);
    let mut answer = 0;

    for line in input.lines() {
        let xs = parse_whitespace_separated_items::<i64>(line);
        calculate_differences(&xs, &mut buf);

        let next = buf[20]
            + buf[40]
            + buf[59]
            + buf[77]
            + buf[94]
            + buf[110]
            + buf[125]
            + buf[139]
            + buf[152]
            + buf[164]
            + buf[175]
            + buf[185]
            + buf[194]
            + buf[202]
            + buf[209]
            + buf[215]
            + buf[220]
            + buf[224]
            + buf[227]
            + buf[229]
            + buf[230];

        answer += next;
    }

    answer
}

pub fn part2(input: &str) -> i64 {
    let mut buf = Vec::with_capacity(231);
    let mut answer = 0;

    for line in input.lines() {
        let xs = parse_whitespace_separated_items::<i64>(line);
        calculate_differences(&xs, &mut buf);

        // `cargo fmt` freezes for me if I don't skip this line :P
        #[rustfmt::skip]
        let prev = buf[0] - (buf[21] - (buf[41] - (buf[60] - (buf[78] - (buf[95] - (buf[111] - (buf[126] - (buf[140] - (buf[153] - (buf[165] - (buf[176] - (buf[186] - (buf[195] - (buf[203] - (buf[210] - (buf[216] - (buf[221] - (buf[225] - (buf[228] - buf[230])))))))))))))))))));

        answer += prev;
    }

    answer
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(
            super::part1(include_str!("../input/day_09.txt")),
            1641934234
        );
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(include_str!("../input/day_09.txt")), 975);
    }
}
