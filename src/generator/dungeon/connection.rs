use crate::generator::math::Vector;
use super::room::Room;
use super::path::Path;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use rand_chacha::ChaCha8Rng;
use rand::{seq::SliceRandom, Rng};

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
            let (position, _) = Connection::get_point_on_room_walls(rng, &self.from);
            // Rules for waypoints:
            // - waypoints never are in the corner of a room
            // - waypoints always have even coordinates
            self.path.waypoints.push(position);

            // TODO - add intermediate waypoints, there should be at least one,
            // serving as the hinge of the path (those waypoint should do a
            // collision check with rooms)

            let (position, _) = Connection::get_point_on_room_walls(rng, &self.to);

            self.path.waypoints.push(position);
        }
    }

    fn get_point_on_room_walls(rng: &mut ChaCha8Rng, room: &Weak<RefCell<Room>>) -> (Vector<i8>, Direction) {
        let rect = room.upgrade().unwrap().borrow().rect.clone();
        let mut position = Vector {
            x: rng.gen_range((rect.p1.x + 1)..(rect.p2.x - 2)),
            y: rng.gen_range((rect.p1.y + 1)..(rect.p2.y - 2)),
        };

        // use even-only values ...
        position.x = if position.x % 2 == 0 { position.x } else { position.x - 1 };
        position.y = if position.y % 2 == 0 { position.y } else { position.y - 1 };

        // ... but not for 'min' corners
        position.x = if position.x == rect.p1.x { position.x + 2 } else { position.x };
        position.y = if position.y == rect.p1.y { position.y + 2 } else { position.y };

        let mut directions: Vec<Direction> = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];

        directions.shuffle(rng);

        let direction = directions[0].clone();

        match direction {
            Direction::North => position.y = rect.p1.y,
            Direction::South => position.y = rect.p2.y - 1,
            Direction::East => position.x = rect.p1.x,
            Direction::West => position.x = rect.p2.x - 1,
        }

        (position, direction)
    }
}

#[derive(Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}
