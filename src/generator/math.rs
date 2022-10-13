#[repr(C)]
#[derive(Clone)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

pub struct Rectangle {
    pub position: Vector<i32>,
    pub size: Vector<u8>,
}

#[allow(dead_code)]
pub fn overlap(_a: &Rectangle, _b: &Rectangle) -> bool {
    false
}
