mod generator;

use generator::map::Map;
use generator::{math::Vector, run};
use libc::{c_char, c_uchar};
use std::ffi::CString;

#[repr(C)]
pub struct Handle {
    _data: Map,
}

#[repr(C)]
pub struct Config {
    pub seed: u64,
    pub rooms_count: usize,
    pub rooms_min_size: Vector<u8>,
    pub rooms_max_size: Vector<u8>,
    pub rooms_spacing: MinMax,
    pub path_extension: MinMax,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MinMax {
    pub min: u8,
    pub max: u8,
}

impl Config {
    pub fn build(
        seed: u64,
        rooms_count: usize,
        min: Vec<u8>,
        max: Vec<u8>,
        spacing: (u8, u8),
        extension: (u8, u8),
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
            rooms_spacing: MinMax { min: spacing.0, max: spacing.1 },
            path_extension: MinMax { min: extension.0, max: extension.1 },
        })
    }

    pub fn new() -> Config {
        Config {
            seed: 0,
            rooms_count: 0,
            rooms_min_size: Vector { x: 0, y: 0 },
            rooms_max_size: Vector { x: 0, y: 0 },
            rooms_spacing: MinMax { min: 0, max: 0 },
            path_extension: MinMax { min: 0, max: 0 },
        }
    }
}

#[no_mangle]
pub extern "C" fn get_config() -> *mut Config {
    Box::into_raw(Box::new(Config::new()))
}

#[no_mangle]
pub extern "C" fn map_create(config: *mut Config) -> *mut Handle {
    let cfg = unsafe { Box::<Config>::from_raw(config) };

    let c = Config::build(
        cfg.seed,
        cfg.rooms_count,
        Vec::from([cfg.rooms_min_size.x, cfg.rooms_min_size.y]),
        Vec::from([cfg.rooms_max_size.x, cfg.rooms_max_size.y]),
        (cfg.rooms_spacing.min, cfg.rooms_spacing.max),
        (cfg.path_extension.min, cfg.path_extension.max),
    );

    drop(cfg);

    Box::into_raw(Box::new(Handle { _data: generate(c.unwrap()) }))
}

#[no_mangle]
pub unsafe extern "C" fn map_destroy(handle: *mut Handle) -> std::os::raw::c_int {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);

        return 0;
    }

    -1
}

#[no_mangle]
pub unsafe extern "C" fn map_size(handle: *mut Handle) -> Vector<u8> {
    if let Some(handle) = handle.as_mut() {
        return Vector { x: handle._data.width, y: handle._data.height };
    }

    Vector { x: 0, y: 0 }
}

#[no_mangle]
pub unsafe extern "C" fn map_as_string(handle: *mut Handle) -> *mut c_char {
    if let Some(handle) = handle.as_mut() {
        let c_str_grid = CString::new(handle._data.to_ascii()).unwrap();

        return c_str_grid.into_raw();
    }

    CString::new("").unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn map_as_bytes(handle: *mut Handle) -> *mut c_uchar {
    if let Some(handle) = handle.as_mut() {
        let mut vec = handle._data.to_bytes();
        let ptr = vec.as_mut_ptr();

        std::mem::forget(vec);

        return ptr;
    }

    Box::into_raw(Box::new(Vec::<u8>::new())) as *mut _
}

pub fn generate(config: Config) -> Map {
    run(
        config.seed,
        config.rooms_count,
        config.rooms_min_size,
        config.rooms_max_size,
        (config.rooms_spacing.min, config.rooms_spacing.max),
        (config.path_extension.min, config.path_extension.max),
    )
}
