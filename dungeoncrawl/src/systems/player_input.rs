use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    let delta = match key {
        Some(VirtualKeyCode::Up) => Point::new(0, -1),
        Some(VirtualKeyCode::Down) => Point::new(0, 1),
        Some(VirtualKeyCode::Left) => Point::new(-1, 0),
        Some(VirtualKeyCode::Right) => Point::new(1, 0),
        _ => return,
    };
    let mut players = <&mut Point>::query().filter(component::<Player>());
    players.iter_mut(ecs).for_each(|pos| {
        let destination = *pos + delta;
        if map.can_enter_tile(destination) {
            *pos = destination;
            camera.on_player_move(destination);
        }
    });
}
