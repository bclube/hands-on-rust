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
    <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|(entity, pos)| {
            let destination = *pos + delta;
            if map.can_enter_tile(destination) {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
                *turn_state = TurnState::PlayerTurn;
            }
        });
}
