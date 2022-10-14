use std::ops::Add;

#[repr(C)]
#[derive(Clone)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub struct Rectangle {
    pub position: Vector<i8>,
    pub size: Vector<u8>,
}

#[allow(dead_code)]
pub fn overlap(_a: &Rectangle, _b: &Rectangle) -> bool {
    false
}
