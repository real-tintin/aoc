use clap::Parser;
use core::panic;

mod day_1;
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

    let ret: u32;

    match args.day {
        1 => match args.part {
            1 => ret = day_1::part_1(args.input),
            2 => ret = day_1::part_2(args.input),
            _ => panic!("Day not implemented"),
        },
        _ => panic!("Day not implemented"),
    }

    println!("{}", ret);
}
