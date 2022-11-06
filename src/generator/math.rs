use std::cmp;
use std::ops::Add;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug)]
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

    pub fn is_inside(&self, p: Vector<i8>) -> bool {
        p.x >= self.p1.x
            && p.x < self.p2.x
            && p.y >= self.p1.y
            && p.y < self.p2.y
    }

    pub fn size(&self) -> Vector<u8> {
        Vector {
            x: (self.p2.x - self.p1.x) as u8,
            y: (self.p2.y - self.p1.y) as u8,
        }
    }
}
