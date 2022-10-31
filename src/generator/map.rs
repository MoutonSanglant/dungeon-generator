use super::math::{Rectangle, Vector};

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

    pub fn to_string(&self) -> String {
        let mut map_string: String = self.grid.clone().into_iter().map(|i| {
            match i {
                Tile::Floor => "x",
                Tile::Door => "o",
                _ => ".",
            }
        }).collect();

        for i in (0..self.grid.len()).step_by(self.width as usize).rev() {
            map_string.insert(i, '\n');
        }

        map_string
    }

    pub fn clear(&mut self) {
        self.grid = Map::new_grid(self.width, self.height);
    }

    pub fn resize(&mut self, min: &Vector<i8>, max: &Vector<i8>) {
        let (mut w, mut h) = self.size();
        let width = u8::try_from(max.x - min.x).ok().unwrap();
        let height = u8::try_from(max.y - min.y).ok().unwrap();

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
        let signed_width = i8::try_from(self.width).ok().unwrap();
        for y in rect.p1.y..rect.p2.y {
            let p1_x = rect.p1.x + self.offset.x;
            let p2_x = rect.p2.x + self.offset.x;
            let y = y + self.offset.y;

            for x in p1_x..p2_x {
                self.grid[usize::try_from(x as u32 + y as u32 * signed_width as u32).ok().unwrap()] = Tile::Floor;
            }
        }
    }

    pub fn add_door(&mut self, position: &Vector<i8>){
        let signed_width = i8::try_from(self.width).ok().unwrap();
        let x = position.x + self.offset.x;
        let y = position.y + self.offset.y;

        self.grid[usize::try_from(x as u32 + y as u32 * signed_width as u32).ok().unwrap()] = Tile::Door;
    }

    fn new_grid(width: u8, height: u8) -> Vec<Tile> {
        vec![Tile::Empty; usize::try_from(width as u32 * height as u32).ok().unwrap()]
    }
}
