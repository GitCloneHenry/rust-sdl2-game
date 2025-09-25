use std::fs;

use crate::physics::{AABB, RigidBody};

#[derive(Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Tile {
    pub tile_type: TileType,
    pub rigid_body: RigidBody,
}

pub struct World {
    pub tiles: Vec<Vec<Tile>>,
}

impl World {
    pub fn from(path: &str) -> Result<Self, String> {
        let contents =
            fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path, e))?;

        let mut tiles = Vec::new();
        for line in contents.lines() {
            let mut row = Vec::new();
            for ch in line.split_whitespace().collect::<Vec<_>>() {
                let tile_type = match ch {
                    "1" => TileType::Wall,
                    "0" => TileType::Floor,
                    _ => return Err(format!("Unexpected character: {}", ch)),
                };
                let tile_rigid_body = RigidBody {
                    AABB { x, y, x + 1, y + 1 }, 
                    true
                }
                let tyle = Tile {
                    tile_type: tile_type,
                    rigid_body: tile_rigid_body,
                }
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
