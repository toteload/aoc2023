mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long)]
    part: u8,

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

    if !(1..=2).contains(&args.part) {
        println!("Invalid part given: {}. Part must be 1 or 2.", args.part);
        return;
    }

    let input = fs::read_to_string(format!("input/day_{:02}_part_{}.txt", args.day, args.part))
        .expect("Should be able to read input text file");

    if args.verbose {
        println!("Input from file:");
        println!("{}", input);
    }

    match (args.day, args.part) {
        (1, 1) => day_01::part_1(&input),
        (1, 2) => day_01::part_2(&input),
        (2, 1) => day_02::part_1(&input),
        (2, 2) => day_02::part_2(&input),
        (3, 1) => day_03::part_1(&input),
        (3, 2) => day_03::part_2(&input),
        (4, 1) => day_04::part_1(&input),
        (4, 2) => day_04::part_2(&input),
        (5, 1) => day_05::part_1(&input),
        (5, 2) => day_05::part_2(&input),
        (6, 1) => day_06::part_1(&input),
        (6, 2) => day_06::part_2(&input),
        (7, 1) => day_07::part_1(&input),
        (7, 2) => day_07::part_2(&input),
        (8, 1) => day_08::part_1(&input),
        (8, 2) => day_08::part_2(&input),
        (9, 1) => day_09::part_1(&input),
        (9, 2) => day_09::part_2(&input),
        (10, 1) => day_10::part_1(&input),
        (10, 2) => day_10::part_2(&input),
        (11, 1) => day_11::part_1(&input),
        (11, 2) => day_11::part_2(&input),
        (12, 1) => day_12::part_1(&input),
        (12, 2) => day_12::part_2(&input),
        (13, 1) => day_13::part_1(&input),
        (13, 2) => day_13::part_2(&input),
        (14, 1) => day_14::part_1(&input),
        (14, 2) => day_14::part_2(&input),
        (15, 1) => day_15::part_1(&input),
        (15, 2) => day_15::part_2(&input),
        (16, 1) => day_16::part_1(&input),
        (16, 2) => day_16::part_2(&input),
        (17, 1) => day_17::part_1(&input),
        (17, 2) => day_17::part_2(&input),
        (18, 1) => day_18::part_1(&input),
        (18, 2) => day_18::part_2(&input),
        (19, 1) => day_19::part_1(&input),
        (19, 2) => day_19::part_2(&input),
        (20, 1) => day_20::part_1(&input),
        (20, 2) => day_20::part_2(&input),
        (21, 1) => day_21::part_1(&input),
        (21, 2) => day_21::part_2(&input),
        (22, 1) => day_22::part_1(&input),
        (22, 2) => day_22::part_2(&input),
        (23, 1) => day_23::part_1(&input),
        (23, 2) => day_23::part_2(&input),
        (24, 1) => day_24::part_1(&input),
        (24, 2) => day_24::part_2(&input),
        (25, 1) => day_25::part_1(&input),
        (25, 2) => day_25::part_2(&input),
        _ => unreachable!(),
    }
}
