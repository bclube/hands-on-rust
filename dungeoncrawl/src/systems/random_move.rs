use crate::prelude::*;
use std::collections::HashMap;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &Map) {
    let others = <(Entity, &Point)>::query()
        .filter(component::<Health>())
        .iter(ecs)
        .map(|(entity, pos)| (pos, *entity))
        .collect::<HashMap<&Point, Entity>>();
    <(Entity, &Point, &MovingRandomly)>::query()
        .iter(ecs)
        .for_each(|(entity, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let destination = loop {
                let destination = match rng.range(0, 4) {
                    0 => Point::new(-1, 0),
                    1 => Point::new(1, 0),
                    2 => Point::new(0, -1),
                    _ => Point::new(0, 1),
                } + *pos;
                if map.can_enter_tile(destination) {
                    break destination;
                }
            };
            if let Some(victim) = others.get(&destination) {
                commands.push((
                    (),
                    WantsToAttack {
                        attacker: *entity,
                        victim: *victim,
                    },
                ));
            } else {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        });
}
