use std::cmp;
use std::ops::Add;

#[repr(C)]
#[derive(Clone, Debug)]
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

#[derive(Debug)]
pub struct Rectangle {
    pub p1: Vector<i8>,
    pub p2: Vector<i8>,
}

impl Rectangle {
    pub fn overlap(&self, other: &Rectangle) -> bool {
        let width_check = cmp::min(self.p2.x, other.p2.x) > cmp::max(self.p1.x, other.p1.x);
        let height_check = cmp::min(self.p2.y, other.p2.y) > cmp::max(self.p1.y, other.p1.y);

        width_check && height_check
    }
}
