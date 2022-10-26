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

impl Dungeon {
    pub fn find_empty_space(&self, size: Vector<i8>) -> Result<Rectangle, PlacementError> {
        let mut rng = self.rng.clone();
        let mut indices: Vec<usize> = (0..self.rooms.len()).collect();

        indices.shuffle(&mut rng);

        for index in indices {
            let mut directions: Vec<u8> = (0..=3).collect();
            let room = self.get_room_at_index(index);

            directions.shuffle(&mut rng);

            for direction in directions {
                let mut position = room.rect.p1.clone();
                let p2 = room.rect.p2.clone();

                match direction {
                    0 => {
                        position = Vector {
                            x: position.x,
                            y: p2.y + 1,
                        }
                    }
                    1 => {
                        position = Vector {
                            x: position.x,
                            y: position.y - (1 + size.y),
                        }
                    }
                    2 => {
                        position = Vector {
                            x: p2.x + 1,
                            y: position.y,
                        }
                    }
                    3 => {
                        position = Vector {
                            x: position.x - (1 + size.x),
                            y: position.y,
                        }
                    }
                    _ => position = Vector { x: 0, y: 0 },
                }

                let rect = Rectangle {
                    p1: position.clone(),
                    p2: position.clone()
                        + Vector {
                            x: size.x,
                            y: size.y,
                        },
                };

                let overlap = self.overlap_any_room(&rect);

                if !overlap {
                    return Ok(rect);
                }
            }
        }

        Err(PlacementError::new("Cannot find a valid position"))
    }

    fn overlap_any_room(&self, rect: &Rectangle) -> bool {
        let mut overlap = false;

        for room in self.rooms.iter() {
            overlap = room.rect.overlap(rect);

            if overlap {
                break;
            }
        }

        overlap
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

    fn get_room_at_index(&self, index: usize) -> &Room {
        &self.rooms[index]
    }

    pub fn get_room_at_index_mut(&mut self, index: usize) -> &mut Room {
        &mut self.rooms[index]
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }
}
