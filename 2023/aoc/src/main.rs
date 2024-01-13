use clap::Parser;
use core::panic;

use crate::types::Part;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod types;
mod utils;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    input: std::path::PathBuf,

    #[arg(short, long)]
    day: u8,

    #[arg(short, long, value_parser = clap::value_parser!(Part))]
    part: Part,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let log_level = if args.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    env_logger::builder()
        .filter_level(log_level)
        .format_target(false)
        .format_timestamp(None)
        .init();

    let ret = match args.day {
        1 => day_1::solve(args.input, args.part),
        2 => day_2::solve(args.input, args.part),
        3 => day_3::solve(args.input, args.part),
        4 => day_4::solve(args.input, args.part),
        _ => panic!("Day not implemented"),
    };

    println!("{}", ret);
}
