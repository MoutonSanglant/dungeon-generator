use super::room::Room;
use std::rc::{Weak, Rc};
use std::cell::RefCell;

pub struct Connection {
    from: Weak<RefCell<Room>>,
    to: Weak<RefCell<Room>>,
}

impl Connection {
    pub fn new(start: &Weak<RefCell<Room>>, room: &Weak<RefCell<Room>>) -> Connection {
        Connection {
            from: Weak::clone(start),
            to: Weak::clone(room),
        }
    }

    pub fn has_destination(&self, room: &Rc<RefCell<Room>>) -> bool {
        Rc::ptr_eq(&self.to.upgrade().unwrap(), room)
    }
}
