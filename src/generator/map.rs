pub struct Map {
    pub width: u8,
    pub height: u8,
    grid: Vec<String>,
}

impl Map {
    pub(super) fn build() -> Map {
        Map {
            width: 0,
            height: 0,
            grid: Map::new_grid(0, 0),
        }
    }

    pub fn size(&self) -> (u8, u8) {
        (self.width, self.height)
    }

    pub fn set_size(&mut self, width: u8, height: u8) {
        self.width = width;
        self.height = height;
        self.clear();
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
