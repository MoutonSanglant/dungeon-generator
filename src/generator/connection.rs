use super::room::Room;
use std::rc::{Weak, Rc};
use std::cell::RefCell;

pub struct Connection {
    destination: Weak<RefCell<Room>>,
}

impl Connection {
    pub fn new(room: &Weak<RefCell<Room>>) -> Connection {
        Connection {
            destination: Weak::clone(room),
        }
    }

    pub fn has_destination(&self, room: &Rc<RefCell<Room>>) -> bool {
        Rc::ptr_eq(&self.destination.upgrade().unwrap(), room)
    }
}
