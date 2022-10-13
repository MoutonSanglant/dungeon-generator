mod generator;

use generator::{math::Vector, run};

#[repr(C)]
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

    pub fn new() -> Config {
        Config {
            rooms_count: 0,
            rooms_min_size: Vector { x: 0, y: 0 },
            rooms_max_size: Vector { x: 0, y: 0 },
        }
    }
}

#[no_mangle]
pub extern "C" fn get_config() -> *mut Config {
    Box::into_raw(Box::new(Config::new()))
}

#[no_mangle]
pub extern "C" fn generate_ext(config: *mut Config) {
    let cfg = unsafe { Box::<Config>::from_raw(config) };

    let c = Config::build(
        cfg.rooms_count,
        Vec::from([cfg.rooms_min_size.x, cfg.rooms_min_size.y]),
        Vec::from([cfg.rooms_max_size.x, cfg.rooms_max_size.y]),
    );

    generate(c.unwrap());

    drop(cfg);
}

pub fn generate(config: Config) {
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
}
