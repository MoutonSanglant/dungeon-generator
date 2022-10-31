use super::room::Room;
use super::path::Path;
use super::math::Vector;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use rand_chacha::ChaCha8Rng;
use rand::Rng;

pub struct Connection {
    from: Weak<RefCell<Room>>,
    to: Weak<RefCell<Room>>,
    pub path: Path,
}

impl Connection {
    pub fn new(from: Weak<RefCell<Room>>, to: Weak<RefCell<Room>>) -> Connection {
        Connection {
            from,
            to,
            path: Path::empty(),
        }
    }

    pub fn has_destination(&self, room: &Rc<RefCell<Room>>) -> bool {
        Rc::ptr_eq(&self.to.upgrade().unwrap(), room)
    }

    pub fn make_path(&mut self, rng: &mut ChaCha8Rng) {
        if self.path.waypoints.len() <= 0
        {
            self.path.waypoints.push(self.select_wall(rng, &self.from));
            // TODO - add intermediate waypoints, there should be at least one,
            // serving as the hinge of the path (those waypoint should do a
            // collision check with rooms)
            self.path.waypoints.push(self.select_wall(rng, &self.to));
        }
    }

    fn select_wall(&self, rng: &mut ChaCha8Rng, room: &Weak<RefCell<Room>>) -> Vector<i8> {
        let rect = room.upgrade().unwrap().borrow().rect.clone();
        let mut position = Vector {
            x: rng.gen_range(rect.p1.x..(rect.p2.x - 1)),
            y: rng.gen_range(rect.p1.y..(rect.p2.y - 1)),
        };

        match rng.gen_range(0..4) {
            0 => position.y = rect.p1.y,
            1 => position.y = rect.p2.y - 1,
            2 => position.x = rect.p1.x,
            3 => position.x = rect.p2.x - 1,
            _ => (),
        }

        position
    }
}
