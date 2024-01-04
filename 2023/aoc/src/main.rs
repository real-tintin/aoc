use clap::Parser;
use core::panic;

use crate::types::Part;

mod day_1;
mod day_2;
mod types;
mod utils;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    input: std::path::PathBuf,

    #[arg(short, long)]
    day: u8,

    #[arg(short, long)]
    part: u8,

    #[arg(short, long)]
    verbose: bool,
}

type Solve = fn(input: std::path::PathBuf, part: Part) -> u32;

fn main() {
    let solve: Solve;
    let ret: u32;

    let args = Args::parse();

    if args.verbose {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .format_target(false)
            .format_timestamp(None)
            .init();
    } else {
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .format_target(false)
            .format_timestamp(None)
            .init();
    }

    match args.day {
        1 => solve = day_1::solve,
        2 => solve = day_2::solve,
        _ => panic!("Day not implemented"),
    }

    match args.part {
        1 => ret = solve(args.input, Part::One),
        2 => ret = solve(args.input, Part::Two),
        _ => panic!("Part not supported"),
    }

    println!("{}", ret);
}
