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

    #[arg(short, long, value_parser = clap::value_parser!(Part))]
    part: Part,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
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

    let ret = match args.day {
        1 => day_1::solve(args.input, args.part),
        2 => day_2::solve(args.input, args.part),
        _ => panic!("Day not implemented"),
    };

    println!("{}", ret);
}
