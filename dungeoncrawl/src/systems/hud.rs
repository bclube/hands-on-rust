use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let (current_health, max_health) = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .map_or((0, 1), |h| (h.current, h.max));
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        current_health,
        max_health,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {} ", current_health, max_health),
        ColorPair::new(WHITE, RED),
    );
    draw_batch.submit(10000).expect("Batch error");
}
