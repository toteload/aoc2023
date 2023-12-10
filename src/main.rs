use std::hint::black_box;
use std::time::{Duration, Instant};

mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: Option<u8>,

    #[arg(short, long, requires("day"), value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,

    #[arg(short, long, default_value_t = false)]
    bench: bool,
}

fn input(day: u8) -> &'static str {
    match day {
        1 => include_str!("../input/day_01.txt"),
        2 => include_str!("../input/day_02.txt"),
        3 => include_str!("../input/day_03.txt"),
        4 => include_str!("../input/day_04.txt"),
        5 => include_str!("../input/day_05.txt"),
        6 => include_str!("../input/day_06.txt"),
        7 => include_str!("../input/day_07.txt"),
        8 => include_str!("../input/day_08.txt"),
        9 => include_str!("../input/day_09.txt"),
        10 => include_str!("../input/day_10.txt"),
        _ => todo!(),
    }
}

fn run(day: u8, part: u8) {
    macro_rules! match_run_day {
        ($day:expr, $day_name:ident) => {
            match_run_day!($day, $day_name, 1, part1);
            match_run_day!($day, $day_name, 2, part2);
        };

        ($day:expr, $day_name:ident, $part:expr, $part_name:ident) => {
            if day == $day && part == $part {
                let answer = $day_name::$part_name(input(day));
                println!("Day {day:2}, part {part} = {answer}");
            }
        };
    }

    match_run_day!(1, day01);
    match_run_day!(2, day02);
    match_run_day!(3, day03);
    match_run_day!(4, day04);
    match_run_day!(5, day05);
    match_run_day!(6, day06);
    match_run_day!(7, day07);
    match_run_day!(8, day08);
    match_run_day!(9, day09);
    match_run_day!(10, day10);
    //match_run_day!(11, day01);
    //match_run_day!(12, day01);
}

fn bench_samples(day: u8, part: u8, sample_count: usize) -> Vec<Duration> {
    macro_rules! match_day {
        ($day:expr, $day_name:ident) => {
            match_day!($day, $day_name, 1, part1);
            match_day!($day, $day_name, 2, part2);
        };

        ($day:expr, $day_name:ident, $part:expr, $part_name:ident) => {
            if day == $day && part == $part {
                let text = input(day);
                let mut samples = Vec::new();

                for _ in 0..sample_count {
                    let start = Instant::now();
                    let _ = black_box($day_name::$part_name(black_box(text)));
                    let t = start.elapsed();
                    samples.push(t);
                }

                return samples;
            }
        };
    }

    match_day!(1, day01);
    match_day!(2, day02);
    match_day!(3, day03);
    match_day!(4, day04);
    match_day!(5, day05);
    match_day!(6, day06);
    match_day!(7, day07);
    match_day!(8, day08);
    match_day!(9, day09);
    match_day!(10, day10);

    todo!()
}

fn print_statistics(samples: &[Duration]) {
    let n = samples.len() as f64;
    let sum = samples.iter().map(|d| d.as_secs_f64()).sum::<f64>();
    let avg = sum / n;
    let stddev = {
        let s = samples.iter().map(|d| (d.as_secs_f64() - avg).powf(2.0)).sum::<f64>();
        (s / n).sqrt()
    };

    print!("avg. time: ");

    if avg > 1.0 {
        print!("{:>6.2}s", avg);
    } else if avg > 0.001 {
        print!("{:>6.2}ms", avg * 1_000.0);
    } else if avg > 0.000_001 {
        print!("{:>6.2}μs", avg * 1_000_000.0);
    } else {
        print!("{:>6.2}ns", avg * 1_000_000_000.0);
    }

    print!(" +- ");
    if stddev > 1.0 {
        print!("{:>6.2}s", stddev);
    } else if stddev > 0.001 {
        print!("{:>6.2}ms", stddev * 1_000.0);
    } else if stddev > 0.000_001 {
        print!("{:>6.2}μs", stddev * 1_000_000.0);
    } else {
        print!("{:>6.2}ns", stddev * 1_000_000_000.0);
    }

    println!("");
}

fn main() {
    let args = Args::parse();

    if args.bench {
        for day in 1..=25 {
            for part in 1..=2 {
                print!("Day {day:2}, part {part} | ");
                let samples = bench_samples(day, part, 10);
                print_statistics(&samples);
            }
        }
        return;
    }

    if args.day.is_none() {
        // Run all the days
        for day in 1..=25 {
            for part in 1..=2 {
                run(day, part);
            }
        }

        return;
    }

    let Some(day) = args.day else { unreachable!() };

    if args.part.is_none() {
        run(day, 1);
        run(day, 2);
        return;
    }

    let Some(part) = args.part else {
        unreachable!()
    };

    run(day, part);
}
