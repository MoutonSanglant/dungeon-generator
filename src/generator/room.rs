use super::math::Rectangle;

pub struct Room {
    pub id: usize,
    pub rect: Rectangle,
    pub connections: Vec<usize>,
}

impl Room {
    pub fn connect_to(&mut self, room_id: usize) -> bool {
        if self.connections.len() >= 4 || self.connections.contains(&room_id) {
            return false;
        }

        // TODO
        // - add waypoints
        // - store wall index (North, South, East, West), use enum
        // - write (in grid)

        self.connections.push(room_id);

        true
    }
}
