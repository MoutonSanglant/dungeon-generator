mod generator;

use generator::{math::Vector, run};
use std::error::Error;

pub struct Config {
    pub rooms_count: usize,
    pub rooms_min_size: Vector<u8>,
    pub rooms_max_size: Vector<u8>,
}

impl Config {
    pub fn build(rooms_count: usize, min: Vec<u8>, max: Vec<u8>) -> Result<Config, &'static str> {
        let min = Vector {
            x: min[0],
            y: min[1],
        };
        let max = Vector {
            x: max[0],
            y: max[1],
        };

        if min.x > max.x || min.y > max.y {
            return Err("Min size cannot be bigger than max size");
        }

        Ok(Config {
            rooms_count,
            rooms_min_size: min,
            rooms_max_size: max,
        })
    }
}

pub fn generate(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Rooms: {}\nMin: {},{}\nMax: {},{}",
        config.rooms_count,
        config.rooms_min_size.x,
        config.rooms_min_size.y,
        config.rooms_max_size.x,
        config.rooms_max_size.y
    );

    run(
        config.rooms_count,
        config.rooms_min_size,
        config.rooms_max_size,
    );

    Ok(())
}
