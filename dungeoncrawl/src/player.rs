use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        let delta = match ctx.key {
            Some(VirtualKeyCode::Left) => Point::new(-1, 0),
            Some(VirtualKeyCode::Right) => Point::new(1, 0),
            Some(VirtualKeyCode::Up) => Point::new(0, -1),
            Some(VirtualKeyCode::Down) => Point::new(0, 1),
            _ => return,
        };
        let new_position = self.position + delta;
        if map.can_enter_tile(new_position) {
            self.position = new_position;
        }
    }
}
