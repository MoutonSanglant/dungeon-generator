use math::{overlap, Rectangle, Vector};
use rand::Rng;
use std::fmt;

pub mod math;

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

struct Dungeon {
    min_size: Vector<u8>,
    max_size: Vector<u8>,
    rooms: Vec<Room>,
}

impl Dungeon {
    fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    fn find_fitting_space(&self, size: &Vector<u8>) -> Vector<i32> {
        Vector { x: 0, y: 0 }
    }
}

pub fn run(rooms: usize, min: Vector<u8>, max: Vector<u8>) {
    println!("Generating dungeon with {} rooms", rooms);

    let r: Vec<Room> = Vec::new();
    let mut dungeon = Dungeon {
        rooms: r,
        min_size: min,
        max_size: max,
    };

    for i in 0..rooms {
        let size = Vector {
            x: rand::thread_rng().gen_range(dungeon.min_size.x..=dungeon.max_size.x),
            y: rand::thread_rng().gen_range(dungeon.min_size.y..=dungeon.max_size.y),
        };
        let position = dungeon.find_fitting_space(&size);
        let rect = Rectangle { size, position };

        dungeon.add_room(Room { id: i, rect });

        println!("{}", dungeon.rooms[i]);
    }
}
