use super::math::{Rectangle, Vector};

pub struct Map {
    pub width: u8,
    pub height: u8,
    offset: Vector<i8>,
    grid: Vec<String>,
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
        self.grid.join("\n")
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
        for y in rect.p1.y..rect.p2.y {
            let p1_x = usize::try_from(rect.p1.x + self.offset.x).ok().unwrap();
            let p2_x = usize::try_from(rect.p2.x + self.offset.x).ok().unwrap();

            self.grid[usize::try_from(y + self.offset.y).ok().unwrap()]
                .replace_range(p1_x..p2_x, &"x".repeat(p2_x - p1_x));
        }
    }

    fn new_grid(width: u8, height: u8) -> Vec<String> {
        vec![String::from(".".repeat(usize::from(width))); usize::from(height)]
    }
}
