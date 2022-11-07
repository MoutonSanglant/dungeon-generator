use clap::Parser;
use std::process::ExitCode;

use dungeon_generator::{generate, Config};

#[derive(Parser)]
#[clap(version, author)]
pub struct Args {
    #[clap(long, default_value = "42", help = "Seed")]
    seed: u64,
    #[clap(
        short,
        long,
        value_parser,
        default_value = "11",
        help = "Number of rooms"
    )]
    rooms: usize,
    #[clap(
        long,
        multiple = true, number_of_values = 2,
        value_parser = clap::value_parser!(u8).range(2..),
        default_values = &["4", "4"],
        help = "Minimum size of a room"
    )]
    min: Vec<u8>,
    #[clap(
        long,
        multiple = true,
        number_of_values = 2,
        value_parser = clap::value_parser!(u8).range(2..),
        default_values = &["7", "7"],
        help = "Maximum size of a room"
    )]
    max: Vec<u8>,
    #[clap(
        long,
        multiple = true,
        number_of_values = 2,
        value_parser = clap::value_parser!(u8).range(2..),
        default_values = &["3", "5"],
        help = "Min & Max spacing between rooms"
    )]
    spacing: (u8, u8),
    #[clap(
        long,
        multiple = true,
        number_of_values = 2,
        value_parser = clap::value_parser!(u8).range(2..),
        default_values = &["2", "4"],
        help = "Min & Max size when extending a path"
    )]
    extension: (u8, u8),
}

fn main() -> ExitCode {
    let args = Args::parse();
    let config = Config::build(args.seed, args.rooms, args.min, args.max, args.spacing, args.extension);

    if let Err(e) = config {
        println!("Process exited with error: {}", e);

        return ExitCode::from(101);
    }

    let map = generate(config.unwrap());

    println!("Generated map:\n{}", map.to_ascii());

    ExitCode::from(0)
}
