use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
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

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let target = loc + delta;
        if self.can_enter_tile(target) {
            Some(map_idx(target.x, target.y))
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let location = self.index_to_point2d(idx);
        [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
        ]
        .iter()
        .filter_map(|delta| self.valid_exit(location, *delta).map(|p| (p, 1.0)))
        .collect()
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] != TileType::Floor
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.in_bounds(pos)
    }
}
