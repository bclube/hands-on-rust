use crate::prelude::*;

pub struct DungeonTheme {}
pub struct ForestTheme {}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Wall => to_cp437('"'),
            TileType::Floor => to_cp437(';'),
        }
    }
}

impl ForestTheme {
    pub fn new_theme() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl DungeonTheme {
    pub fn new_theme() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Wall => to_cp437('#'),
            TileType::Floor => to_cp437('.'),
        }
    }
}
