mod automata;
mod drunkard;
mod rooms;

use std::{collections::HashSet, iter};

use crate::prelude::*;
use automata::CellularAutomataArchitect;
use drunkard::DrunkardsWalkArchitect;
use rooms::RoomsArchitect;

const NUM_ROOMS: usize = 20;

trait MapArchitect {
    fn new_map(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl Default for MapBuilder {
    fn default() -> Self {
        Self {
            map: Map::new(),
            rooms: Default::default(),
            monster_spawns: Default::default(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        }
    }
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(RoomsArchitect {}),
            1 => Box::new(CellularAutomataArchitect {}),
            _ => Box::new(DrunkardsWalkArchitect {}),
        };
        architect.new_map(rng)
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.fill(tile);
    }

    fn find_most_distant(&self) -> Point {
        const UNREACHABLE: &f32 = &f32::MAX;

        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &[self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if (1..SCREEN_WIDTH).contains(&p.x) && (1..SCREEN_HEIGHT).contains(&p.y) {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_map_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_map_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| {
            let a_sum = a.center().x + a.center().y;
            let b_sum = b.center().x + b.center().y;
            a_sum.cmp(&b_sum)
        });
        let diff = 1;
        for (i, room) in rooms.iter().enumerate().skip(diff) {
            let prev = rooms[i - diff].center();
            let new = room.center();

            match rng.range(0, 2) {
                0 => {
                    self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                    self.apply_vertical_tunnel(prev.y, new.y, new.x);
                }
                _ => {
                    self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                    self.apply_horizontal_tunnel(prev.x, new.x, new.y);
                }
            }
        }
    }

    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;

        let spawnable_tiles = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter_map(|(idx, t)| {
                let pos = self.map.index_to_point2d(idx);
                if *t == TileType::Floor && DistanceAlg::Pythagoras.distance2d(*start, pos) > 10.0 {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut seen = HashSet::new();
        iter::repeat_with(|| rng.random_slice_index(&spawnable_tiles).unwrap())
            .filter_map(|v| {
                if seen.insert(v) {
                    Some(spawnable_tiles[v])
                } else {
                    None
                }
            })
            .take(NUM_MONSTERS)
            .collect()
    }

    pub fn add_outside_walls(&mut self) {
        for y in 0..SCREEN_HEIGHT {
            self.map.tiles[map_idx(0, y)] = TileType::Wall;
            self.map.tiles[map_idx(SCREEN_WIDTH - 1, y)] = TileType::Wall;
        }
        for x in 0..SCREEN_WIDTH {
            self.map.tiles[map_idx(x, 0)] = TileType::Wall;
            self.map.tiles[map_idx(x, SCREEN_HEIGHT - 1)] = TileType::Wall;
        }
    }
}
