use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
pub fn hud(
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] key: &Option<VirtualKeyCode>,
    ecs: &SubWorld,
) {
    let ((current_health, max_health), pos) = <(&Health, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .map_or(((0, 1), Point::zero()), |(h, p)| ((h.current, h.max), *p));
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
    draw_batch.submit(30000).expect("Batch error");

    if let Some(VirtualKeyCode::M) = key {
        let player_idx = map_idx(pos.x, pos.y);
        let search_targets = vec![player_idx];
        let dijkstra_map =
            DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);
        let offset = Point::new(camera.left_x, camera.top_y);
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(2);
        dijkstra_map.map.iter().enumerate().for_each(|(i, d)| {
            if *d < 100.0 {
                let screen_pos = map.index_to_point2d(i);
                if camera.in_frame(screen_pos) {
                    draw_batch.print((screen_pos - offset) * 4, d.to_string());
                }
            }
        });
        draw_batch.submit(20000).expect("Batch error");
    };
}
