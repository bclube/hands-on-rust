use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let (player_entity, player_pos) = match <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
    {
        Some(pos) => pos,
        _ => return,
    };
    let player_idx = map_idx(player_pos.x, player_pos.y);
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);
    <(Entity, &Point)>::query()
        .filter(component::<ChasingPlayer>())
        .iter(ecs)
        .for_each(|(entity, pos)| {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            if distance < 1.2 {
                commands.push((
                    (),
                    WantsToAttack {
                        attacker: *entity,
                        victim: *player_entity,
                    },
                ));
            } else {
                let idx = map_idx(pos.x, pos.y);
                if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *entity,
                            destination: map.index_to_point2d(destination),
                        },
                    ));
                }
            }
        });
}
