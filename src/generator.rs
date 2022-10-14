use map::Map;
use math::{overlap, Rectangle, Vector};
use rand::{seq::SliceRandom, thread_rng, Rng};
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
            "Room {} (pos: [{}; {}], size: [{}; {}])",
            self.id, self.rect.position.x, self.rect.position.y, self.rect.size.x, self.rect.size.y
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
    map: Map,
}

impl Dungeon {
    // TODO - poorly named, because self is mutable, it's not a "find" method anymore
    fn find_room_position(&mut self, size: Vector<u8>) -> Vector<i8> {
        let index = self.get_room_index(rand::thread_rng().gen_range(0..self.rooms.0.len()));

        // TODO - Loop through rooms starting at index, until a good room as been found

        loop {
            // TODO - improvment idea: cache remaining directions in the room struct
            let mut directions: Vec<u8> = (0..=3).collect();

            directions.shuffle(&mut thread_rng());

            for _i in directions {
                /* TODO - do overlap check
                 * if true => check next direction
                 * else => good position, return it
                 */

                // TODO - Change the position
                let position = Vector { x: 0, y: 0 };
                let overlap = overlap(
                    &self.rooms.0[index].rect,
                    &Rectangle {
                        position: position.clone(),
                        size: size.clone(),
                    },
                );

                if !overlap {
                    let size = position.clone()
                        + Vector {
                            x: i8::try_from(size.x).ok().unwrap(),
                            y: i8::try_from(size.y).ok().unwrap(),
                        };

                    self.set_map_bounds(&position, &size);

                    return position;
                }
            }

            break;
        }

        Vector { x: 0, y: 0 }
    }

    fn set_map_bounds(&mut self, min: &Vector<i8>, max: &Vector<i8>) {
        let (mut w, mut h) = self.map.size();
        let width = u8::try_from(max.x - min.x).ok().unwrap();
        let height = u8::try_from(max.y - min.y).ok().unwrap();

        if width > w {
            w = width;
        }

        if height > h {
            h = height;
        }

        self.map.set_size(w, h);
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

// TODO - add a seed parameter
pub fn run(rooms: usize, min: Vector<u8>, max: Vector<u8>) -> Map {
    let mut dungeon = Dungeon {
        rooms: Rooms(Vec::new()),
        connections: Connections(Vec::new()),
        min_size: min,
        max_size: max,
        map: Map::build(),
    };

    for i in 0..rooms {
        let size = Vector {
            x: rand::thread_rng().gen_range(dungeon.min_size.x..=dungeon.max_size.x),
            y: rand::thread_rng().gen_range(dungeon.min_size.y..=dungeon.max_size.y),
        };
        let position = if i > 0 {
            dungeon.find_room_position(size.clone())
        } else {
            Vector { x: 0, y: 0 }
        };
        let rect = Rectangle { size, position };

        dungeon.add_room(Room { id: i, rect });

        if i > 0 {
            for _j in 0..rand::thread_rng().gen_range(1..4) {
                dungeon.connect_rooms(i, rand::thread_rng().gen_range(0..i));
            }
        }
    }

    dungeon.map
}
