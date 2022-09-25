use clap::Parser;
use std::process::ExitCode;

use dungeon_generator::{generate, Config};

#[derive(Parser)]
#[clap(version, author)]
pub struct Args {
    #[clap(
        short,
        long,
        value_parser,
        default_value = "3",
        help = "Number of rooms"
    )]
    rooms: usize,
    #[clap(
        long,
        multiple = true, number_of_values = 2,
        value_parser = clap::value_parser!(u8).range(2..),
        default_values = &["2", "2"],
        help = "Minimum size of a room"
    )]
    min: Vec<u8>,
    #[clap(
        long,
        multiple = true,
        number_of_values = 2,
        value_parser = clap::value_parser!(u8).range(2..),
        default_values = &["5", "5"],
        help = "Maximum size of a room"
    )]
    max: Vec<u8>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let config = Config::build(args.rooms, args.min, args.max);

    if let Err(e) = config {
        println!("Process exited with error: {}", e);

        return ExitCode::from(101);
    }

    generate(config.unwrap());

    ExitCode::from(0)
}
