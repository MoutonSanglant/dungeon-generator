pub mod map;
pub mod math;

mod dungeon;
mod errors;
mod room;

use dungeon::Dungeon;
use map::Map;
use math::{Rectangle, Vector};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use room::Room;

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
