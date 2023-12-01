use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::is_digit,
    combinator::{map, value, verify},
    IResult,
};

const INPUT: &str = include_str!("../../input/day_01.txt");

fn try_digit(b: &u8) -> Option<u32> {
if b.is_ascii_digit() {
                Some((b - b'0') as u32)
            } else {
                None
            }
}

fn part_1(input: &str) {
    let mut answer = 0;

    for line in input.lines() {
        let a = line.as_bytes().iter().filter_map(try_digit).next().unwrap();
        let b = line.as_bytes().iter().rev().filter_map(try_digit).next().unwrap();

        answer += a * 10 + b;
    }

    println!("{answer}");
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

fn part_2(input: &str) {
    let mut answer = 0;

    for line in input.lines() {

        let a = {
            let mut y = 0;
            for start in 0..line.len() {
                let res = alt((a_digit, written_number))(&line[start..]);
                if let Ok((_, x)) = res {
                    y = x;
                    break;
                }
            }

            y
        };

        let b = {
            let mut y = 0;
            for start in (0..line.len()).rev() {
                let res = alt((a_digit, written_number))(&line[start..]);
                if let Ok((_, x)) = res {
                    y = x;
                    break;
                }
            }

            y
        };

        answer += a * 10 + b;
    }

    println!("{answer}")
}

fn main() {
    part_1(INPUT);
    part_2(INPUT);
}
