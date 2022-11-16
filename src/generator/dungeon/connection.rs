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

    pub fn make_path(&mut self, rng: &mut ChaCha8Rng, path_extension: (u8, u8)) {
        if self.path.waypoints.len() <= 0 {
            let (from_pos, from_dir) = Connection::create_room_exit(rng, &self.from, &self.to, path_extension).unwrap();
            let (to_pos, _to_dir) = Connection::create_room_exit(rng, &self.to, &self.from, path_extension).unwrap();

            let mut path = Connection::find_path(
                from_pos.clone(),
                from_dir.clone(),
                to_pos.clone(),
                &self.to.upgrade().unwrap().borrow().rect,
                path_extension,
                rng,
            );

            self.path.waypoints.push(to_pos);
            self.path.waypoints.append(&mut path);
            self.path.waypoints.push(from_pos);
        }
    }

    /// Find a path going from one point to another, avoiding penetration into the destination room
    fn find_path(from_pos: Vector<i8>, from_dir: Direction, to_pos: Vector<i8>, to_rect: &Rectangle, path_extension: (u8, u8), rng: &mut ChaCha8Rng) -> Vec<Vector<i8>> {
        let rot = match from_dir {
            Direction::North => Rotation2::identity(),
            Direction::South => Rotation2::new(std::f32::consts::FRAC_PI_2 * 2.0),
            Direction::East => Rotation2::new(-std::f32::consts::FRAC_PI_2),
            Direction::West => Rotation2::new(std::f32::consts::FRAC_PI_2),
        };
        let pos_from = rot * Point2::new(from_pos.x as f32, from_pos.y as f32);
        let pos_to = rot * Point2::new(to_pos.x as f32, to_pos.y as f32);
        let mut path = Vec::new();

        match Connection::find_next_waypoint(&mut path, pos_from, pos_to, to_rect, rot.inverse(), 0, path_extension, rng) {
            _ => path,
        }
    }

    /// Find the next waypoint of the path.
    /// This method assumes points are rotated toward North (Y-)
    fn find_next_waypoint(path: &mut Vec<Vector<i8>>, pos_from: Point2<f32>, pos_to: Point2<f32>, rect_to: &Rectangle, inv: Rotation2<f32>, iteration: i8, path_extension: (u8, u8), rng: &mut ChaCha8Rng) -> Option<bool> {
        if iteration > 10 {
            return None;
        }
        let delta = pos_from.y - pos_to.y;
        let mut pos_next = Point2::new(pos_from.x, pos_from.y);

        if delta <= 0f32 {
            pos_next.y -= rng.gen_range(path_extension.0..path_extension.1) as f32;
        }
        else if iteration > 0
        {
            pos_next.y -= delta;
            let world_to = inv * Point2::new(pos_next.x, pos_next.y + 1f32);

            if rect_to.is_inside(Vector { x: world_to.x.round() as i8, y: world_to.y.round() as i8 }) {
                return Some(false);
            }
            else if relative_eq!(pos_next, pos_to) {
                return Some(true);
            }
        }

        let rot_delta = pos_from.x - pos_to.x;
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
            match Connection::find_next_waypoint(path, next_rot * pos_next, next_rot * pos_to, rect_to, inv * next_rot.inverse(), iteration + 1, path_extension, rng) {
                Some(true) => {
                    let world_point = inv * pos_next;
                    let mut pos = Vector { x: world_point.x.round() as i8, y: world_point.y.round() as i8 };
                    // align the point to even cells on grid, it works because
                    // the same transform is applied to all points in the path
                    pos.x = if pos.x % 2 == 0 { pos.x } else { pos.x - 1 };
                    pos.y = if pos.y % 2 == 0 { pos.y } else { pos.y - 1 };
                    path.push(pos);
                    break;
                },
                Some(false) => {
                    pos_next.y -= rng.gen_range(path_extension.0..path_extension.1) as f32;
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
    fn create_room_exit(rng: &mut ChaCha8Rng, room: &Weak<RefCell<Room>>, other_room: &Weak<RefCell<Room>>, path_extension: (u8, u8)) -> Option<(Vector<i8>, Direction)> {
        let rect = room.upgrade().unwrap().borrow().rect.clone();
        let other_rect = other_room.upgrade().unwrap().borrow().rect.clone();
        let coords = Vector {
            x: rng.gen_range((rect.p1.x + 1)..(rect.p2.x - 2)),
            y: rng.gen_range((rect.p1.y + 1)..(rect.p2.y - 2)),
        };

        // ... and avoid corner coords
        //coords.x = if coords.x == rect.p1.x { coords.x + 2 } else { coords.x };
        //coords.y = if coords.y == rect.p1.y { coords.y + 2 } else { coords.y };

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

            // align the point to even cells on grid
            position.x = if position.x % 2 == 0 { position.x } else { position.x - 1 };
            position.y = if position.y % 2 == 0 { position.y } else { position.y - 1 };

            let limit = (path_extension.1 + 1) as i8;
            let test_point = match direction {
                Direction::North => Vector { x: position.x, y: position.y - limit },
                Direction::South => Vector { x: position.x, y: position.y + limit },
                Direction::East => Vector { x: position.x + limit, y: position.y },
                Direction::West => Vector { x: position.x - limit, y: position.y },
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
