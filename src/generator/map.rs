use super::Dungeon;
use std::fmt;

pub struct Map {
    pub width: u8,
    pub height: u8,
    grid: Vec<String>,
}

impl Map {
    pub(super) fn build(dungeon: Dungeon) -> Map {
        let (width, height) = dungeon.get_size();

        Map {
            width,
            height,
            grid: Map::new_grid(width, height),
        }
    }

    pub fn to_string(&self) -> String {
        self.grid.join("\n")
    }

    pub fn clear(&mut self) {
        self.grid = Map::new_grid(self.width, self.height);
    }

    fn new_grid(width: u8, height: u8) -> Vec<String> {
        vec![String::from(".".repeat(usize::from(width))); usize::from(height)]
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.grid.iter().fold(Ok(()), |result, line| {
            result.and_then(|_| writeln!(f, "{}", line))
        })
    }
}
