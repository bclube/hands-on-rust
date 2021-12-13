use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let delta = match key {
        Some(VirtualKeyCode::Up) => Point::new(0, -1),
        Some(VirtualKeyCode::Down) => Point::new(0, 1),
        Some(VirtualKeyCode::Left) => Point::new(-1, 0),
        Some(VirtualKeyCode::Right) => Point::new(1, 0),
        Some(VirtualKeyCode::Space) => {
            *turn_state = TurnState::PlayerTurn;
            return;
        }
        _ => return,
    };
    let (player_entity, destination) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
        .unwrap();
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
