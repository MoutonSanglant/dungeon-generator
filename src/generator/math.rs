pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

pub struct Rectangle {
    pub position: Vector<i32>,
    pub size: Vector<u8>,
}

pub fn overlap(a: &Rectangle, b: &Rectangle) -> bool {
    false
}
