use errors::PlacementError;
use map::Map;
use math::{Rectangle, Vector};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub mod errors;
pub mod map;
pub mod math;

struct Room {
    id: usize,
    rect: Rectangle,
    connections: Vec<usize>,
}

impl Room {
    fn connect_to(&mut self, room_id: usize) {
        if self.connections.contains(&room_id) {
            return;
        }

        // TODO
        // - add waypoints
        // - store wall index (North, South, East, West), use enum
        // - write (in grid)

        self.connections.push(room_id);
    }
}

struct Dungeon {
    min_size: Vector<u8>,
    max_size: Vector<u8>,
    rooms: Vec<Room>,
    rng: ChaCha8Rng,
}

impl Dungeon {
    fn find_empty_space(&self, size: Vector<i8>) -> Result<Rectangle, PlacementError> {
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

    fn to_map(&self) -> Map {
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

    fn get_room_at_index_mut(&mut self, index: usize) -> &mut Room {
        &mut self.rooms[index]
    }

    fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }
}

pub fn run(seed: u64, rooms: usize, min: Vector<u8>, max: Vector<u8>) -> Map {
    let mut dungeon = Dungeon {
        rooms: Vec::new(),
        min_size: min,
        max_size: max,
        rng: ChaCha8Rng::seed_from_u64(seed),
    };

    if dungeon.max_size.x > 127 || dungeon.max_size.y > 127 {
        panic!("Room size must be in the range [0,128)")
    }

    for i in 0..rooms {
        add_room(&mut dungeon, i);

        if i < 1 {
            continue;
        }

        for _j in 0..dungeon.rng.gen_range(1..=4) {
            let other_id = dungeon.rng.gen_range(0..i);
            let room = dungeon.get_room_at_index_mut(i);

            room.connect_to(other_id);
        }
    }

    dungeon.to_map()
}

fn add_room(dungeon: &mut Dungeon, id: usize) {
    let signed_size = Vector {
        x: dungeon
            .rng
            .gen_range(dungeon.min_size.x..=dungeon.max_size.x) as i8,
        y: dungeon
            .rng
            .gen_range(dungeon.min_size.y..=dungeon.max_size.y) as i8,
    };

    let rect = if id == 0 {
        Rectangle {
            p1: Vector { x: 0, y: 0 },
            p2: Vector {
                x: signed_size.x,
                y: signed_size.y,
            },
        }
    } else {
        match dungeon.find_empty_space(signed_size.clone()) {
            Ok(rect) => rect,
            Err(_error) => Rectangle {
                p1: Vector { x: 0, y: 0 },
                p2: Vector { x: 0, y: 0 },
            }, // We don't care, the room will be discarded
               //Err(error) => panic!("Cannot construct dungeon: {:?}", error),
        }
    };

    dungeon.add_room(Room {
        id,
        rect,
        connections: Vec::new(),
    });
}
