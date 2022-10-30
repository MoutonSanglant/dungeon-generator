use super::errors::PlacementError;
use super::map::Map;
use super::math::{Rectangle, Vector};
use super::room::Room;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

pub struct Dungeon {
    pub min_size: Vector<u8>,
    pub max_size: Vector<u8>,
    pub rooms: Vec<Room>,
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
            let room = &self.rooms[index];
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

        match direction {
            Direction::North => position.y = p2.y + 1,
            Direction::East  => position.x = p2.x + 1,
            Direction::South => position.y = position.y - (1 + size.y),
            Direction::West  => position.x = position.x - (1 + size.x),
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

        for room in self.rooms.iter() {
            min.x = if room.rect.p1.x < min.x {
                room.rect.p1.x
            } else {
                min.x
            };

            min.y = if room.rect.p1.y < min.y {
                room.rect.p1.y
            } else {
                min.y
            };

            max.x = if room.rect.p2.x > max.x {
                room.rect.p2.x
            } else {
                max.x
            };

            max.y = if room.rect.p2.y > max.y {
                room.rect.p2.y
            } else {
                max.y
            };
        }

        let mut map = Map::build();

        map.resize(&min, &max);

        for room in self.rooms.iter() {
            map.add_room(&room.rect);
        }

        map
    }

    pub fn connect_rooms(&mut self, first: usize, second: usize) -> bool  {
        if !self.rooms[first].can_connect_to(second) ||
            !self.rooms[second].can_connect_to(first)
        {
            return false;
        }

        self.rooms[first].connect_to(second);
        self.rooms[second].connect_to(first);

        true
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    fn overlap_test(&self, rect: &Rectangle) -> bool {
        let mut overlap = false;

        for room in self.rooms.iter() {
            overlap = room.rect.overlap(&rect);

            if overlap {
                break;
            }
        }

        overlap
    }
}
