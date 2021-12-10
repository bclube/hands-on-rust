use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        let floor_tile = to_cp437('.');
        let wall_tile = to_cp437('#');
        let mut idx = 0;
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                //let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Wall => ctx.set(x, y, GREEN, BLACK, wall_tile),
                    TileType::Floor => ctx.set(x, y, YELLOW, BLACK, floor_tile),
                }
                idx += 1;
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        (0..SCREEN_WIDTH).contains(&point.x) && (0..SCREEN_HEIGHT).contains(&point.y)
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_map_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}