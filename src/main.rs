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
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(long)]
    day: u8,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    if !(1..=25).contains(&args.day) {
        println!(
            "Invalid day given: {}. Given day must be between 1 and 25 inclusive.",
            args.day
        );
        return;
    }

    let input = fs::read_to_string(format!("input/{:02}.txt", args.day))
        .expect("Should be able to read input text file");

    if args.verbose {
        println!("Input from file:");
        println!("{}", input);
    }

    match args.day {
        1 => day01::run(&input),
        2 => day02::run(&input),
        3 => day03::run(&input),
        4 => day04::run(&input),
        5 => day05::run(&input),
        6 => day06::run(&input),
        7 => day07::run(&input),
        8 => day08::run(&input),
        9 => day09::run(&input),
        10 => day10::run(&input),
        11 => day11::run(&input),
        12 => day12::run(&input),
        13 => day13::run(&input),
        14 => day14::run(&input),
        15 => day15::run(&input),
        16 => day16::run(&input),
        17 => day17::run(&input),
        18 => day18::run(&input),
        19 => day19::run(&input),
        20 => day20::run(&input),
        21 => day21::run(&input),
        22 => day22::run(&input),
        23 => day23::run(&input),
        24 => day24::run(&input),
        25 => day25::run(&input),
        _ => unreachable!(),
    }
}
