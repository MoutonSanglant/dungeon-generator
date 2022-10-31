use crate::generator::math::Vector;

pub struct Path {
    pub waypoints: Vec<Vector<i8>>,
}

impl Path {
    pub fn empty() -> Path {
        Path {
            waypoints: vec![],
        }
    }
}
