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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;

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
        11 => include_str!("../input/day_11.txt"),
        12 => include_str!("../input/day_12.txt"),
        13 => include_str!("../input/day_13.txt"),
        14 => include_str!("../input/day_14.txt"),
        15 => include_str!("../input/day_15.txt"),
        16 => include_str!("../input/day_16.txt"),
        17 => include_str!("../input/day_17.txt"),
        18 => include_str!("../input/day_18.txt"),
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
                return;
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
    match_run_day!(11, day11);
    match_run_day!(12, day12);
    match_run_day!(13, day13);
    match_run_day!(14, day14);
    match_run_day!(15, day15);
    match_run_day!(16, day16);
    match_run_day!(17, day17);
    match_run_day!(18, day18);

    todo!()
}

fn bench(day: u8, part: u8, sample_count: usize) -> Vec<Duration> {
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
    match_day!(11, day11);
    match_day!(12, day12);
    match_day!(13, day13);
    match_day!(14, day14);
    match_day!(15, day15);
    match_day!(16, day16);
    match_day!(17, day17);
    match_day!(18, day18);

    todo!()
}

fn print_time(secs: f64) {
    if secs > 1.0 {
        print!("{:>3.0}s", secs);
    } else if secs > 0.001 {
        print!("{:>3.0}ms", secs * 1_000.0);
    } else if secs > 0.000_001 {
        print!("{:>3.0}μs", secs * 1_000_000.0);
    } else {
        print!("{:>3.0}ns", secs * 1_000_000_000.0);
    }
}

#[derive(Debug)]
struct Benchmark {
    name: String,
    mean: f64,
    stddev: f64,
    percentage: f64,
}

impl Benchmark {
    fn from_samples(name: String, samples: Vec<Duration>) -> Benchmark {
        let mut samples = samples;
        samples.sort();

        // Only keep the best 90% of samples to remove outliers.
        samples.truncate((samples.len() * 10) / 9);

        let n = samples.len() as f64;
        let sum = samples.iter().map(|d| d.as_secs_f64()).sum::<f64>();
        let mean = sum / n;
        let stddev = {
            let s = samples
                .iter()
                .map(|d| (d.as_secs_f64() - mean).powf(2.0))
                .sum::<f64>();
            (s / n).sqrt()
        };

        Benchmark {
            name,
            mean,
            stddev,
            percentage: 0.0,
        }
    }

    fn print(&self) {
        print!("{} | ", self.name);
        print_time(self.mean);
        print!(" +- ");
        print_time(self.stddev);
        println!(" | {:>4.1}%", self.percentage);
    }
}

fn main() {
    let args = Args::parse();

    let sample_count = 10;
    let max_day = 16;

    if args.bench {
        let mut benchmarks = Vec::new();

        for day in 1..=max_day {
            for part in 1..=2 {
                let samples = bench(day, part, sample_count);
                benchmarks.push(Benchmark::from_samples(
                    format!("Day {day:2}, part {part}"),
                    samples,
                ));
            }
        }

        let total_time = benchmarks.iter().fold(0.0, |acc, x| acc + x.mean);

        for x in benchmarks.iter_mut() {
            x.percentage = x.mean * 100.0 / total_time;
        }

        for x in benchmarks {
            x.print();
        }

        print!("\nTotal time taken: ");
        print_time(total_time);
        println!();

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
