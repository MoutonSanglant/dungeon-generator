mod generator;

use generator::map::Map;
use generator::{math::Vector, run};
use libc::c_char;
use std::ffi::CString;

#[repr(C)]
pub struct CMap {
    width: u8,
    height: u8,
    grid: *mut c_char,
}

#[repr(C)]
pub struct Config {
    pub seed: u64,
    pub rooms_count: usize,
    pub rooms_min_size: Vector<u8>,
    pub rooms_max_size: Vector<u8>,
}

impl Config {
    pub fn build(
        seed: u64,
        rooms_count: usize,
        min: Vec<u8>,
        max: Vec<u8>,
    ) -> Result<Config, &'static str> {
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
            seed,
            rooms_count,
            rooms_min_size: min,
            rooms_max_size: max,
        })
    }

    pub fn new() -> Config {
        Config {
            seed: 0,
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
pub extern "C" fn generate_ext(config: *mut Config) -> *mut CMap {
    let cfg = unsafe { Box::<Config>::from_raw(config) };

    let c = Config::build(
        cfg.seed,
        cfg.rooms_count,
        Vec::from([cfg.rooms_min_size.x, cfg.rooms_min_size.y]),
        Vec::from([cfg.rooms_max_size.x, cfg.rooms_max_size.y]),
    );

    drop(cfg);

    let map = generate(c.unwrap());
    let c_str_grid = CString::new(map.to_string()).unwrap();

    Box::into_raw(Box::new(CMap {
        width: map.width,
        height: map.height,
        grid: c_str_grid.into_raw(),
    }))
}

pub fn generate(config: Config) -> Map {
    run(
        config.seed,
        config.rooms_count,
        config.rooms_min_size,
        config.rooms_max_size,
    )
}
