use super::math::Rectangle;
use super::connection::Connection;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Room {
    pub id: usize,
    pub rect: Rectangle,
    pub connections: Vec<Connection>,
}

impl Room {
    pub fn is_connected_to(&self, room: &Rc<RefCell<Room>>) -> bool {
        self.connections.iter().any(|c| c.has_destination(room))
    }

    pub fn add_connection(&mut self, room: &Rc<RefCell<Room>>) {
        self.connections.push(Connection::new(&Rc::downgrade(room)));
    }
}
