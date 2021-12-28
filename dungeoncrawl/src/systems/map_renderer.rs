use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Point)]
#[read_component(Player)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let (player_fov, player_pos) = <(&FieldOfView, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    let floor_glyph = to_cp437('.');
    let wall_glyph = to_cp437('#');
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = map_idx(x, y);
            let tint = if player_fov.visible_tiles.contains(&pt) {
                distance_tint(*player_pos, pt, player_fov.radius, WHITE, GRAY30)
            } else {
                GRAY30
            };
            if map.in_bounds(pt)
                && (player_fov.visible_tiles.contains(&pt) || map.revealed_tiles[idx])
            {
                let glyph = match map.tiles[idx] {
                    TileType::Floor => floor_glyph,
                    TileType::Wall => wall_glyph,
                };
                draw_batch.set(pt - offset, ColorPair::new(tint, BLACK), glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
