use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut player_query = <(Entity, &Point)>::query().filter(component::<Player>());
    let delta = match key {
        Some(VirtualKeyCode::Up) => Point::new(0, -1),
        Some(VirtualKeyCode::Down) => Point::new(0, 1),
        Some(VirtualKeyCode::Left) => Point::new(-1, 0),
        Some(VirtualKeyCode::Right) => Point::new(1, 0),
        Some(VirtualKeyCode::Space) => {
            <&mut Health>::query()
                .filter(component::<Player>())
                .iter_mut(ecs)
                .for_each(|health| health.current = health.max.min(health.current + 1));
            *turn_state = TurnState::PlayerTurn;
            return;
        }
        Some(VirtualKeyCode::P) => {
            *turn_state = TurnState::PlayerTurn;
            return;
        }
        Some(VirtualKeyCode::G) => {
            if let Some((player, player_pos)) = player_query.iter(ecs).next() {
                <(Entity, &Point)>::query()
                    .filter(component::<Item>())
                    .iter(ecs)
                    .filter(|(_entity, &item_pos)| item_pos == *player_pos)
                    .for_each(|(entity, _item_pos)| {
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried(*player));
                    })
            };
            return;
        }
        _ => return,
    };
    if let Some((player_entity, destination)) = player_query
        .iter(ecs)
        .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
    {
        if !map.can_enter_tile(destination) {
            return;
        }
        *turn_state = TurnState::PlayerTurn;
        if let Some(enemy) = <(Entity, &Point)>::query()
            .filter(component::<Enemy>())
            .iter(ecs)
            .filter(|(_, pos)| **pos == destination)
            .find_map(|(entity, pos)| {
                if *pos == destination {
                    Some(entity)
                } else {
                    None
                }
            })
        {
            commands.push((
                (),
                WantsToAttack {
                    attacker: player_entity,
                    victim: *enemy,
                },
            ));
        } else {
            commands.push((
                (),
                WantsToMove {
                    entity: player_entity,
                    destination,
                },
            ));
        }
    }
}
