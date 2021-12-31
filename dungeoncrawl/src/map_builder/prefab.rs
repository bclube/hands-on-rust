use crate::prelude::*;

const FORTRESS: (&str, i32, i32) = (
    "
............
...######...
...#....#...
...#.M..#...
.###....###.
..M......M..
.###....###.
...#....#...
...#....#...
...######...
............
",
    12,
    11,
);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &[mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    for _ in 0..10 {
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - FORTRESS.1),
            rng.range(0, SCREEN_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2,
        );

        if !dimensions.point_in_rect(mb.amulet_start)
            && dimensions.point_set().iter().any(|pt| {
                let idx = mb.map.point2d_to_index(*pt);
                let distance = dijkstra_map.map[idx];
                distance < 2000.0 && distance > 20.0
            })
        {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|pt| !points.contains(pt));
            break;
        }
    }

    if let Some(placement) = placement {
        let string_vec = FORTRESS
            .0
            .chars()
            .filter(|a| !a.is_whitespace())
            .collect::<Vec<_>>();
        let mut i = 0;
        for ty in placement.y..placement.y + FORTRESS.2 {
            for tx in placement.x..placement.x + FORTRESS.1 {
                let idx = map_idx(tx, ty);
                let c = string_vec[i];
                match c {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(tx, ty));
                    }
                    '.' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => panic!("Invalid map character: {:#?}", c),
                }
                i += 1;
            }
        }
    }
}
