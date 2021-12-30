use super::MapArchitect;
use crate::prelude::*;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn new_map(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb: MapBuilder = Default::default();
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distant();
        mb.monster_spawns = mb.rooms.iter().skip(1).map(|room| room.center()).collect();
        mb
    }
}
