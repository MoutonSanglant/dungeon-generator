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
    spacing: Vec<u8>,
    #[clap(
        long,
        multiple = true,
        number_of_values = 2,
        value_parser = clap::value_parser!(u8).range(2..),
        default_values = &["2", "4"],
        help = "Min & Max size when extending a path"
    )]
    extension: Vec<u8>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let config = Config::build(args.seed,
                               args.rooms,
                               args.min,
                               args.max,
                               (args.spacing[0], args.spacing[1]),
                               (args.extension[0], args.extension[1]),
                            );

    if let Err(e) = config {
        println!("Process exited with error: {}", e);

        return ExitCode::from(101);
    }

    println!("Map seed: {}", args.seed);
    println!("Number of rooms: {}", args.rooms);

    let map = generate(config.unwrap());
    let bytes = map.to_bytes();
    let width = map.width as i32;

    println!("Map size: {}x{}", map.width, map.height);
    println!("");
    println!("Generated map (ASCII): {}", map.to_ascii());
    println!("");
    println!("Generated map (bytes):");

    for y in 0..map.height {
        let y = y as i32;
        let from = (y * width) as usize;
        let to = (y * width + width) as usize;

        println!("{}", &bytes[from..to].into_iter().map(|i| i.to_string()).collect::<String>());
    }

    ExitCode::from(0)
}
