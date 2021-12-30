use crate::prelude::*;

pub fn distance_tint(
    from: Point,
    to: Point,
    max: i32,
    near_color: (u8, u8, u8),
    far_color: (u8, u8, u8),
) -> (u8, u8, u8) {
    let max = (max * max) as f32;
    let distance = DistanceAlg::PythagorasSquared.distance2d(from, to);
    blend_color(distance / max, far_color, near_color)
}

fn blend_color(fraction: f32, far: (u8, u8, u8), near: (u8, u8, u8)) -> (u8, u8, u8) {
    (
        blend(fraction, far.0, near.0),
        blend(fraction, far.1, near.1),
        blend(fraction, far.2, near.2),
    )
}

fn blend(fraction: f32, far: u8, near: u8) -> u8 {
    let diff = far as f32 - near as f32;
    let result = near as f32 + fraction * diff;
    let min = far.min(near);
    let max = far.max(near);
    (result as u8).clamp(min, max)
}
