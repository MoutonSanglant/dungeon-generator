pub mod map;
pub mod math;

mod dungeon;
mod errors;

use dungeon::Dungeon;
use map::Map;
use math::{Rectangle, Vector};
use rand::{Rng, SeedableRng};
use rand::distributions::{WeightedIndex, Distribution};
use rand_chacha::ChaCha8Rng;

pub fn run(seed: u64,
           rooms: usize,
           min: Vector<u8>,
           max: Vector<u8>,
           spacing: (u8, u8),
           extension: (u8, u8),
        ) -> Map {
    let mut dungeon = Dungeon {
        rooms: Vec::new(),
        min_size: min,
        max_size: max,
        rooms_spacing: (spacing.0, spacing.1),
        path_extension: (extension.0, extension.1),
        rng: ChaCha8Rng::seed_from_u64(seed),
    };

    if dungeon.max_size.x > 127 || dungeon.max_size.y > 127 {
        panic!("Room size must be in the range [0,128)")
    }

    let mut rng = dungeon.rng.clone();
    let choices = [1, 2, 3, 4];
    let weights = [100, 5, 2, 1];
    let dist = WeightedIndex::new(&weights).unwrap();

    for i in 0..rooms {
        add_room(&mut dungeon, i);

        if i < 1 {
            continue;
        }

        let choice = choices[dist.sample(&mut rng)];
        let connections = rng.gen_range(1..=i.min(choice));
        let other_ids = (0..connections).map(|_| rng.gen_range(0..i));

        for other_id in other_ids {
            dungeon.connect_rooms(i, other_id);
        }
    }

    dungeon.make_paths();
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
        let mut p2 = signed_size;
        // align the point to odd cells on grid
        p2.x = if p2.x % 2 == 0 { p2.x + 1 } else { p2.x };
        p2.y = if p2.y % 2 == 0 { p2.y + 1 } else { p2.y };

        Rectangle {
            p1: Vector { x: 0, y: 0 },
            p2,
        }
    } else {
        match dungeon.find_empty_space(signed_size) {
            Ok(rect) => rect,
            Err(_error) => Rectangle {
                p1: Vector { x: 0, y: 0 },
                p2: Vector { x: 0, y: 0 },
            }, // We don't care, the room will be discarded
               //Err(error) => panic!("Cannot construct dungeon: {:?}", error),
        }
    };

    dungeon.add_room(id, rect);
}
