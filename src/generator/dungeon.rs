mod room;
mod connection;
mod path;

use super::errors::PlacementError;
use super::map::Map;
use super::math::{Rectangle, Vector};
use room::Room;
use rand::{seq::SliceRandom, Rng};
use rand_chacha::ChaCha8Rng;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Dungeon {
    pub min_size: Vector<u8>,
    pub max_size: Vector<u8>,
    pub rooms_spacing: (u8, u8),
    pub path_extension: (u8, u8),
    pub rooms: Vec<Rc<RefCell<Room>>>,
    pub rng: ChaCha8Rng,
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Dungeon {
    pub fn find_empty_space(&self, size: Vector<i8>) -> Result<Rectangle, PlacementError> {
        let mut rng = self.rng.clone();
        let mut indices: Vec<usize> = (0..self.rooms.len()).collect();

        indices.shuffle(&mut rng);

        for index in indices {
            let room = self.rooms[index].borrow();
            let mut directions: Vec<Direction> = vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ];

            directions.shuffle(&mut rng);

            for direction in directions {
                let rect = self.get_rectangle(room.rect.clone(), size.clone(), direction);

                if !self.overlap_test(&rect) {
                    return Ok(rect);
                }
            }
        }

        Err(PlacementError::new("Cannot find a valid position"))
    }

    fn get_rectangle(&self, rect: Rectangle, size: Vector<i8>, direction: Direction) -> Rectangle {
        let mut position = rect.p1;
        let p2 = rect.p2;
        let spacing = self.rng.clone().gen_range(self.rooms_spacing.0..self.rooms_spacing.1) as i8;

        match direction {
            Direction::North => position.y = p2.y + spacing,
            Direction::East  => position.x = p2.x + spacing,
            Direction::South => position.y = position.y - (spacing + size.y),
            Direction::West  => position.x = position.x - (spacing + size.x),
        }

        Rectangle {
            p1: position.clone(),
            p2: position.clone()
                + Vector {
                    x: size.x,
                    y: size.y,
                },
        }
    }

    pub fn to_map(&self) -> Map {
        let mut min = Vector { x: 0, y: 0 };
        let mut max = Vector { x: 0, y: 0 };

        for r in self.rooms.iter() {
            let room = r.borrow_mut();

            (min, max) = self.get_min_max(min, max, &room.rect.p1, &room.rect.p2);

            for connection_ref in room.connections.iter() {
                let connection = connection_ref.borrow_mut();

                for waypoint in connection.path.waypoints.iter() {
                    (min, max) = self.get_min_max(min, max, waypoint, waypoint);
                }

            }
        }

        let mut map = Map::build();

        map.resize(&min, &max);

        for room in self.rooms.iter() {
            map.add_room(&room.borrow().rect);
            for connection in &room.borrow().connections {
                let waypoints = &connection.borrow().path.waypoints;
                let len = waypoints.len();
                for i in 0..len {
                    if i == len - 1 {
                        map.add_door(&waypoints[i]);
                        break;
                    }
                    else {
                        map.add_corridor(&waypoints[i], &waypoints[i + 1]);

                        if i == 0 {
                            map.add_door(&waypoints[i]);
                        }
                    }
                }
            }
        }

        map
    }

    pub fn connect_rooms(&mut self, first: usize, second: usize) -> bool {
        Room::connect(&self.rooms[first], &self.rooms[second])
    }

    pub fn add_room(&mut self, id: usize, rect: Rectangle) {
        let room = Room {
            id,
            rect,
            connections: Vec::new(),
        };
        self.rooms.push(Rc::new(RefCell::new(room)));
    }

    pub fn make_paths(&mut self) {
        for room in self.rooms.iter() {
            for connection in room.borrow().connections.iter() {
                connection.borrow_mut().make_path(&mut self.rng, self.path_extension);
            }
        }
    }

    fn overlap_test(&self, rect: &Rectangle) -> bool {
        let mut overlap = false;

        for r in self.rooms.iter() {
            let room = r.borrow();
            overlap = room.rect.overlap(&rect);

            if overlap {
                break;
            }
        }

        overlap
    }

    fn get_min_max(&self, mut min: Vector<i8>, mut max: Vector<i8>, p1: &Vector<i8>, p2: &Vector<i8>) -> (Vector<i8>, Vector<i8>) {
        min.x = if p1.x < min.x {
            p1.x
        } else {
            min.x
        };

        min.y = if p1.y < min.y {
            p1.y
        } else {
            min.y
        };

        max.x = if p2.x > max.x {
            p2.x
        } else {
            max.x
        };

        max.y = if p2.y > max.y {
            p2.y
        } else {
            max.y
        };

        (min, max)
    }
}
