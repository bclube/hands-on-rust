use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let (player_fov, player_pos) = <(&FieldOfView, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    <(&Point, &Render)>::query()
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(pos, render)| {
            let tint = distance_tint(*player_pos, *pos, player_fov.radius, WHITE, GRAY30);
            draw_batch.set(*pos - offset, ColorPair::new(tint, BLACK), render.glyph);
        });
    draw_batch.submit(5000).expect("Batch error");
}
