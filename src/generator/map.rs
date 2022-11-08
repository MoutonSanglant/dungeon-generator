use super::math::{Rectangle, Vector};
use std::cmp;

pub struct Map {
    pub width: u8,
    pub height: u8,
    offset: Vector<i8>,
    grid: Vec<Tile>,
}

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Floor,
    Door,
    Corridor,
}

impl Map {
    pub(super) fn build() -> Map {
        Map {
            width: 0,
            height: 0,
            offset: Vector { x: 0, y: 0 },
            grid: Map::new_grid(0, 0),
        }
    }

    pub fn size(&self) -> (u8, u8) {
        (self.width, self.height)
    }

    pub fn to_ascii(&self) -> String {
        let mut map_string: String = self.grid.clone().into_iter().map(|i| {
            match i {
                Tile::Floor => "x",
                Tile::Corridor => "#",
                Tile::Door => "o",
                _ => ".",
            }
        }).collect();

        for i in (0..self.grid.len()).step_by(self.width as usize).rev() {
            map_string.insert(i, '\n');
        }

        map_string
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let map_bytes: Vec<u8> = self.grid.clone().into_iter().map(|i| {
            match i {
                Tile::Floor => 1,
                Tile::Corridor => 2,
                Tile::Door => 3,
                _ => 0,
            }
        }).collect();

        map_bytes
    }

    pub fn clear(&mut self) {
        self.grid = Map::new_grid(self.width as u32, self.height as u32);
    }

    pub fn resize(&mut self, min: &Vector<i8>, max: &Vector<i8>) {
        let (mut w, mut h) = self.size();
        let width = (max.x - min.x) as u8;
        let height = (max.y - min.y) as u8;

        if width > w {
            w = width;
        }

        if height > h {
            h = height;
        }

        self.offset = Vector {
            x: if min.x < 0 { -min.x } else { 0 },
            y: if min.y < 0 { -min.y } else { 0 },
        };

        self.width = w + 1;
        self.height = h + 1;
        self.clear();
    }

    pub fn add_room(&mut self, rect: &Rectangle) {
        for y in rect.p1.y..rect.p2.y {
            let p1_x = (rect.p1.x + self.offset.x) as u32;
            let p2_x = (rect.p2.x + self.offset.x) as u32;
            let y = (y + self.offset.y) as u32;

            for x in p1_x..p2_x {
                self.grid[(x + y * self.width as u32) as usize] = Tile::Floor;
            }
        }
    }

    pub fn add_door(&mut self, position: &Vector<i8>) {
        let x = (position.x + self.offset.x) as u32;
        let y = (position.y + self.offset.y) as u32;

        self.grid[(x + y * self.width as u32) as usize] = Tile::Door;
    }

    pub fn add_corridor(&mut self, from: &Vector<i8>, to: &Vector<i8>) {
        if from.x == to.x {
            let min_y = cmp::min(from.y, to.y);
            let max_y = cmp::max(from.y, to.y);
            for y in min_y..=max_y {
                let x = (from.x + self.offset.x) as u32;
                let y = (y + self.offset.y) as u32;
                self.grid[(x + y * self.width as u32) as usize] = Tile::Corridor;
            }
        } else {
            let min_x = cmp::min(from.x, to.x);
            let max_x = cmp::max(from.x, to.x);
            for x in min_x..=max_x {
                let x = (x + self.offset.x) as u32;
                let y = (from.y + self.offset.y) as u32;
                self.grid[(x + y * self.width as u32) as usize] = Tile::Corridor;
            }
        }
    }

    fn new_grid(width: u32, height: u32) -> Vec<Tile> {
        vec![Tile::Empty; (width * height) as usize]
    }
}
