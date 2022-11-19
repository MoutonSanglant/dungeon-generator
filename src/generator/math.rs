extern crate nalgebra as na;

use std::cmp;
use std::ops::Add;
use na::Point2;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    /// Test if a segment intersects with the rectangle
    /// This is an adaptation (in order to work with grid-aligned values)
    /// of the following algorithm:
    /// https://stackoverflow.com/a/293052/2321532
    pub fn intersects(&self, p1: Point2<f32>, p2: Point2<f32>) -> bool {
        let bl = Point2::new((self.p1.x - 1) as f32, (self.p1.y - 1) as f32);
        let br = Point2::new(self.p2.x as f32, (self.p1.y - 1) as f32);
        let tr = Point2::new(self.p2.x as f32, self.p2.y as f32);
        let tl = Point2::new((self.p1.x - 1) as f32, self.p2.y as f32);

        let d1 = Rectangle::point_line_distance(tl, p1, p2);
        let d2 = Rectangle::point_line_distance(tr, p1, p2);
        let d3 = Rectangle::point_line_distance(br, p1, p2);
        let d4 = Rectangle::point_line_distance(bl, p1, p2);

        // If distance are all negative or all positive, there is no intersection
        if (d1 <= 0f32 && d2 <= 0f32 && d3 <= 0f32 && d4 <= 0f32)
            || (d1 >= 0f32 && d2 >= 0f32 && d3 >= 0f32 && d4 >= 0f32)
        {
            return false;
        }

        // Project endpoints onto x & y axis and check if segment shadow
        // intersects the polygon's shadow
        if p1.x > tr.x && p2.x > tr.x
        {
            return false;
        }

        if p1.x < bl.x && p2.x < bl.x
        {
            return false;
        }

        if p1.y > tr.y && p2.y > tr.y
        {
            return false;
        }

        if p1.y < bl.y && p2.y < bl.y
        {
            return false;
        }

        true
    }

    /// Returns the distance of a point to a line
    /// If f(x, y) = 0, point is on the line
    /// If f(x, y) > 0, point is "above" the line
    /// If f(x, y) < 0, point is "below" the line
    fn point_line_distance(p: Point2<f32>, p1: Point2<f32>, p2: Point2<f32>) -> f32 {
        (p2.y - p1.y) * p.x + (p1.x - p2.x) * p.y + (p2.x * p1.y - p1.x * p2.y)
    }
}

// Test if a segment intersect with one rectangle or another
pub fn intersects(p1: Point2<f32>, p2: Point2<f32>, r1: &Rectangle, r2: &Rectangle) -> bool {
    let p1 = Point2::new(p1.x.round(), p1.y.round());
    let p2 = Point2::new(p2.x.round(), p2.y.round());

    r1.intersects(p1, p2) || r2.intersects(p1, p2)
}
