pub mod part1;
pub mod part2;

use clap::Parser;
use std::path::Path;

/// program to solve day4 of 2023 Advent of Code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// name of the input file, should exist in resources
    #[arg(index(1))]
    file: String,

    /// which part of day 4 to solve, either 1 or 2
    #[arg(index(2))]
    part: u8,
}

fn main() {
    let args = Args::parse();

    let binding = ("resources/".to_owned() + &args.file).to_owned();

    eprintln!("\n`{}'\n", binding);
    let path = Path::new(&binding);

    println!(
        "{}\n",
        if args.part == 1 {
            part1::lowest_seed_location(path)
        } else if args.part == 2 {
            part2::lowest_seed_location(path)
        } else {
            panic!("invalid part: `{}'", args.part)
        }
    )
}
