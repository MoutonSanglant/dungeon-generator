extern crate nalgebra as na;
extern crate approx;

use crate::generator::math::{Rectangle, Vector};
use super::room::Room;
use super::path::Path;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use rand_chacha::ChaCha8Rng;
use rand::{seq::SliceRandom, Rng};
use na::{Point2, Rotation2};
use approx::relative_eq;

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
            let (from_pos, from_dir) = Connection::create_room_exit(rng, &self.from, &self.to).unwrap();
            let (to_pos, _to_dir) = Connection::create_room_exit(rng, &self.to, &self.from).unwrap();

            let mut path = Connection::find_path(
                from_pos.clone(),
                from_dir.clone(),
                to_pos.clone(),
                &self.to.upgrade().unwrap().borrow().rect,
                rng
            );

            self.path.waypoints.push(to_pos);
            self.path.waypoints.append(&mut path);
            self.path.waypoints.push(from_pos);
        }
    }

    /// Find a path going from one point to another, avoiding penetration into the destination room
    fn find_path(from_pos: Vector<i8>, from_dir: Direction, to_pos: Vector<i8>, to_rect: &Rectangle, rng: &mut ChaCha8Rng) -> Vec<Vector<i8>> {
        let rot = match from_dir {
            Direction::North => Rotation2::identity(),
            Direction::South => Rotation2::new(std::f32::consts::FRAC_PI_2 * 2.0),
            Direction::East => Rotation2::new(-std::f32::consts::FRAC_PI_2),
            Direction::West => Rotation2::new(std::f32::consts::FRAC_PI_2),
        };
        let pos_from = rot * Point2::new(from_pos.x as f32, from_pos.y as f32);
        let pos_to = rot * Point2::new(to_pos.x as f32, to_pos.y as f32);
        let mut path = Vec::new();

        match Connection::find_next_waypoint(&mut path, pos_from, pos_to, to_rect, rot.inverse(), 0, rng) {
            _ => path,
        }
    }

    /// Find the next waypoint of the path.
    /// This method assumes points are rotated toward North (Y-)
    fn find_next_waypoint(path: &mut Vec<Vector<i8>>, p: Point2<f32>, pos_to: Point2<f32>, rect_to: &Rectangle, inv: Rotation2<f32>, iteration: i8, rng: &mut ChaCha8Rng) -> Option<bool> {
        if iteration > 10 {
            return None;
        }

        let delta = p.y - pos_to.y;
        let mut p2 = Point2::new(p.x, p.y);

        if delta <= 0f32 {
            p2.y -= 2.0; // TODO - use a random range between 2 and 5
        }
        else
        {
            p2.y -= delta;
            let world_to = inv * Point2::new(p2.x, p2.y + 1f32);

            if rect_to.is_inside(Vector { x: world_to.x.round() as i8, y: world_to.y.round() as i8 }) {
                p2.y = p.y - (delta / 2.0);

                return Some(false);
            }
            else if relative_eq!(p2, pos_to) {
                return Some(true);
            }
        }

        let rot_delta = p.x - pos_to.x;
        let next_rot = if relative_eq!(rot_delta, 0f32) {
            let r = rng.gen_range(0..1);
            if r == 0 {
                Rotation2::new(-std::f32::consts::FRAC_PI_2)
            }
            else {
                Rotation2::new(-std::f32::consts::FRAC_PI_2)
            }
        }
        else if rot_delta < 0f32 {
            Rotation2::new(-std::f32::consts::FRAC_PI_2)
        } else {
            Rotation2::new(std::f32::consts::FRAC_PI_2)
        };

        let mut retry = 0;

        loop {
            match Connection::find_next_waypoint(path, next_rot * p2, next_rot * pos_to, rect_to, inv * next_rot.inverse(), iteration + 1, rng) {
                Some(true) => {
                    let world_point = inv * p2;
                    let pos = Vector { x: world_point.x.round() as i8, y: world_point.y.round() as i8 };
                    path.push(pos);
                    break;
                },
                Some(false) => {
                    p2.y -= 2.0;
                }
                _ => {
                    return None;
                },
            }

            retry += 1;

            if retry > 5 {
                return None;
            }
        }

        Some(true)
    }

    /// Create an exit on one wall of a room, the exit cannot face the other room.
    /// The exit will always be on a wall, on an even tile of the grid and cannot be a corner
    fn create_room_exit(rng: &mut ChaCha8Rng, room: &Weak<RefCell<Room>>, other_room: &Weak<RefCell<Room>>) -> Option<(Vector<i8>, Direction)> {
        let rect = room.upgrade().unwrap().borrow().rect.clone();
        let other_rect = other_room.upgrade().unwrap().borrow().rect.clone();
        let mut coords = Vector {
            x: rng.gen_range((rect.p1.x + 1)..(rect.p2.x - 2)),
            y: rng.gen_range((rect.p1.y + 1)..(rect.p2.y - 2)),
        };

        // use even-only values ...
        coords.x = if coords.x % 2 == 0 { coords.x } else { coords.x - 1 };
        coords.y = if coords.y % 2 == 0 { coords.y } else { coords.y - 1 };

        // ... and avoid corner coordss
        coords.x = if coords.x == rect.p1.x { coords.x + 2 } else { coords.x };
        coords.y = if coords.y == rect.p1.y { coords.y + 2 } else { coords.y };

        let mut directions: Vec<Direction> = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];

        directions.shuffle(rng);

        for direction in directions {
            let mut position = coords.clone();

            match direction {
                Direction::North => position.y = rect.p1.y,
                Direction::South => position.y = rect.p2.y - 1,
                Direction::East => position.x = rect.p2.x - 1,
                Direction::West => position.x = rect.p1.x,
            }

            let test_point = match direction {
                Direction::North => Vector { x: position.x, y: position.y - 2 },
                Direction::South => Vector { x: position.x, y: position.y + 2 },
                Direction::East => Vector { x: position.x + 2, y: position.y },
                Direction::West => Vector { x: position.x - 2, y: position.y },
            };

            if !other_rect.is_inside(test_point) {
                return Some((position, direction));
            }
        }

        None
    }
}

#[derive(Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}
