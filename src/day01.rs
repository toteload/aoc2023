use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::is_digit,
    combinator::{map, value, verify},
    IResult,
};

fn try_digit(b: &u8) -> Option<u32> {
    if b.is_ascii_digit() {
        Some((b - b'0') as u32)
    } else {
        None
    }
}

pub fn part1(input: &str) -> u32 {
    let mut answer = 0;

    for line in input.lines() {
        let a = line.as_bytes().iter().filter_map(try_digit).next().unwrap();
        let b = line
            .as_bytes()
            .iter()
            .rev()
            .filter_map(try_digit)
            .next()
            .unwrap();

        answer += a * 10 + b;
    }

    answer
}

fn written_number(input: &str) -> IResult<&str, u32> {
    alt((
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

fn a_digit(input: &str) -> IResult<&str, u32> {
    map(
        // There is probably an easier way to read one digit :P
        verify(take(1usize), |s: &str| is_digit(s.as_bytes()[0])),
        |b: &str| (b.as_bytes()[0] - b'0') as u32,
    )(input)
}

pub fn part2(input: &str) -> u32 {
    let mut answer = 0;

    for line in input.lines() {
        let a = 'search: {
            for start in 0..line.len() {
                let res = alt((a_digit, written_number))(&line[start..]);
                if let Ok((_, x)) = res {
                    break 'search x;
                }
            }

            unreachable!();
        };

        let b = 'search: {
            for start in (0..line.len()).rev() {
                let res = alt((a_digit, written_number))(&line[start..]);
                if let Ok((_, x)) = res {
                    break 'search x;
                }
            }

            unreachable!();
        };

        answer += a * 10 + b;
    }

    answer
}
