use super::MapArchitect;
use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new_map(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb: MapBuilder = Default::default();
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.fill(TileType::Wall);
        self.drunkard(&center, rng, &mut mb.map);
        mb.add_outside_walls();
        mb.player_start = center;
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut floor_tile_count = 0;
        let mut drunkard_pos = *start;

        while floor_tile_count < DESIRED_FLOOR {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            let tile = map.tiles.get_mut(drunk_idx).unwrap();
            if *tile == TileType::Wall {
                floor_tile_count += 1;
            }
            *tile = TileType::Floor;
            match rng.range(0, 4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }
            if !map.in_bounds(drunkard_pos) {
                drunkard_pos = Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT));
            }
        }
    }
}
