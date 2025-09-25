use std::fs;

#[derive(Copy, Clone)]
pub enum Tile {
    Wall, 
    Floor,
}

pub struct World {
    pub tiles: Vec<Vec<Tile>>
}

impl World {
    pub fn from(path: &str) -> Result<Self, String> {
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path, e))?;

        let mut tiles = Vec::new();
        for line in contents.lines() {
            let mut row = Vec::new();
            for ch in line.split_whitespace().collect::<Vec<_>>() {
                let tile = match ch {
                    "1" => Tile::Wall,
                    "0" => Tile::Floor,
                    _ => return Err(format!("Unexpected character: {}", ch)),
                };
                row.push(tile);
            }
            tiles.push(row);
        }

        Ok(World { tiles })
    }

    pub fn _height(&self) -> usize {
        self.tiles.len()
    }

    pub fn _width(&self) -> usize {
        self.tiles.get(0).map_or(0, |row| row.len())
    }
}
