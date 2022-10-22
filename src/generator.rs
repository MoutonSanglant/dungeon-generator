use map::Map;
use math::{Rectangle, Vector};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::fmt;

pub mod map;
pub mod math;

struct Rooms(pub Vec<Room>);

struct Room {
    id: usize,
    rect: Rectangle,
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Room {} (p1: [{}; {}], p2: [{}; {}])",
            self.id, self.rect.p1.x, self.rect.p1.y, self.rect.p2.x, self.rect.p2.y
        )
    }
}

impl fmt::Display for Rooms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, room| {
            result.and_then(|_| writeln!(f, "{}", room))
        })
    }
}

struct Connections(pub Vec<Connection>);

#[derive(PartialEq)]
struct Connection {
    from_index: usize,
    to_index: usize,
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Room {} is connected with room {}",
            self.from_index, self.to_index
        )
    }
}

impl fmt::Display for Connections {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, connection| {
            result.and_then(|_| writeln!(f, "{}", connection))
        })
    }
}

struct Dungeon {
    min_size: Vector<u8>,
    max_size: Vector<u8>,
    rooms: Rooms,
    connections: Connections,
    rng: ChaCha8Rng,
}

impl Dungeon {
    fn find_empty_space(&self, size: Vector<u8>) -> Rectangle {
        let mut rng = self.rng.clone();
        let room_id = rng.gen_range(0..self.rooms.0.len());
        let index = self.get_room_index(room_id);

        loop {
            // TODO - improvment idea: cache free directions in the room struct
            let mut directions: Vec<u8> = (0..=3).collect();

            // TODO - get next index when no room found
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
                            y: position.y - (1 + i8::try_from(size.y).ok().unwrap()),
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
                            x: position.x - (1 + i8::try_from(size.x).ok().unwrap()),
                            y: position.y,
                        }
                    }
                    _ => position = Vector { x: 0, y: 0 },
                }

                let rect = Rectangle {
                    p1: position.clone(),
                    p2: position.clone()
                        + Vector {
                            x: i8::try_from(size.x).ok().unwrap(),
                            y: i8::try_from(size.y).ok().unwrap(),
                        },
                };

                let overlap = self.overlap_any_room(&rect);

                if !overlap {
                    return rect;
                } else {
                    println!("Overlap, try with another position");
                }
            }

            break;
        }

        Rectangle {
            p1: Vector { x: 0, y: 0 },
            p2: Vector { x: 0, y: 0 },
        }
    }

    fn overlap_any_room(&self, rect: &Rectangle) -> bool {
        let mut overlap = false;

        for room in self.rooms.0.iter() {
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

        for room in self.rooms.0.iter() {
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

        for room in self.rooms.0.iter() {
            map.add_room(&room.rect);
        }

        map
    }

    fn get_room_at_index(&self, index: usize) -> &Room {
        &self.rooms.0[index]
    }

    fn get_room_index(&self, id: usize) -> usize {
        self.rooms.0.iter().position(|r| r.id == id).unwrap()
    }

    fn add_room(&mut self, room: Room) {
        self.rooms.0.push(room);
    }

    fn connect_rooms(&mut self, from_index: usize, to_index: usize) {
        if self.connections.0.contains(&Connection {
            from_index,
            to_index,
        }) {
            return;
        }
        self.connections.0.push(Connection {
            from_index,
            to_index,
        });
    }
}

pub fn run(seed: u64, rooms: usize, min: Vector<u8>, max: Vector<u8>) -> Map {
    let mut dungeon = Dungeon {
        rooms: Rooms(Vec::new()),
        connections: Connections(Vec::new()),
        min_size: min,
        max_size: max,
        rng: ChaCha8Rng::seed_from_u64(seed),
    };

    if dungeon.max_size.x > 127 || dungeon.max_size.y > 127 {
        panic!("Room size must be between 0 and 127 (inclusive)")
    }

    for i in 0..rooms {
        add_room(&mut dungeon, i);

        /*
        for _j in 0..rand::thread_rng().gen_range(1..4) {
            dungeon.connect_rooms(i, rand::thread_rng().gen_range(0..i));
        }
        */
    }

    dungeon.to_map()
}

fn add_room(dungeon: &mut Dungeon, id: usize) {
    let size = Vector {
        x: dungeon
            .rng
            .gen_range(dungeon.min_size.x..=dungeon.max_size.x),
        y: dungeon
            .rng
            .gen_range(dungeon.min_size.y..=dungeon.max_size.y),
    };

    let rect = if id == 0 {
        Rectangle {
            p1: Vector { x: 0, y: 0 },
            p2: Vector {
                x: i8::try_from(size.x).ok().unwrap(),
                y: i8::try_from(size.y).ok().unwrap(),
            },
        }
    } else {
        dungeon.find_empty_space(size.clone())
    };

    dungeon.add_room(Room { id, rect });
}
