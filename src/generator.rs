use math::{Rectangle, Vector};
use rand::Rng;
use std::fmt;

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
}

impl Dungeon {
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

    fn find_room_position(&self, _size: &Vector<u8>) -> Vector<i32> {
        Vector { x: 0, y: 0 }
    }
}

impl fmt::Display for Dungeon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rooms:\n{}\nConnections:\n{}",
            self.rooms, self.connections
        )
    }
}

pub fn run(rooms: usize, min: Vector<u8>, max: Vector<u8>) {
    println!("Generating dungeon with {} rooms", rooms);

    let mut dungeon = Dungeon {
        rooms: Rooms(Vec::new()),
        connections: Connections(Vec::new()),
        min_size: min,
        max_size: max,
    };

    for i in 0..rooms {
        let size = Vector {
            x: rand::thread_rng().gen_range(dungeon.min_size.x..=dungeon.max_size.x),
            y: rand::thread_rng().gen_range(dungeon.min_size.y..=dungeon.max_size.y),
        };
        let position = dungeon.find_room_position(&size);
        let rect = Rectangle { size, position };

        dungeon.add_room(Room { id: i, rect });

        if i > 0 {
            for _j in 0..rand::thread_rng().gen_range(1..4) {
                dungeon.connect_rooms(i, rand::thread_rng().gen_range(0..i));
            }
        }
    }

    println!("{}", dungeon);
}
