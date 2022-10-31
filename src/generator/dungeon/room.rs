use crate::generator::math::Rectangle;
use super::connection::Connection;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Room {
    pub id: usize,
    pub rect: Rectangle,
    pub connections: Vec<Rc<RefCell<Connection>>>,
}

impl Room {
    pub fn is_connected_to(&self, room: &Rc<RefCell<Room>>) -> bool {
        self.connections.iter().any(|c| c.borrow().has_destination(room))
    }

    pub fn connect(a: &Rc<RefCell<Room>>, b: &Rc<RefCell<Room>>) -> bool {
        if a.borrow().is_connected_to(b)
            || b.borrow().is_connected_to(a) {
                return false;
        }

        let connection = Rc::new(RefCell::new(Connection::new(Rc::downgrade(a).clone(), Rc::downgrade(b).clone())));

        a.borrow_mut().connections.push(connection.clone());
        b.borrow_mut().connections.push(connection.clone());

        true
    }
}
