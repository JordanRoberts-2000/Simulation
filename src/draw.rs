use piston_window::types::Color;
use piston_window::*;
use piston_window::{ellipse, line, Context, G2d};
use std::f64::consts::PI;

pub fn draw_circle(color: Color, position: [u64; 2], size: u64, c: &Context, g: &mut G2d) {
    // - size / 2 makes the position parameter the center of the circle
    if position[0] < size / 2 || position[1] < size / 2 {
        panic!("draw_circle: attempted to spawn too close to border");
    }
    ellipse(
        color,
        [
            (position[0] - size / 2) as f64,
            (position[1] - size / 2) as f64,
            size as f64,
            size as f64,
        ],
        c.transform,
        g,
    );
}

pub fn draw_hollow_circle(
    context: &Context,
    graphics: &mut G2d,
    color: [f32; 4],
    center: [f64; 2],
    radius: f64,
    segments: usize,
) {
    let (cx, cy) = (center[0], center[1]);
    let step = 2.0 * PI / segments as f64;

    for i in 0..segments {
        let theta1 = i as f64 * step;
        let theta2 = (i + 1) as f64 * step;
        let x1 = cx + radius * theta1.cos();
        let y1 = cy + radius * theta1.sin();
        let x2 = cx + radius * theta2.cos();
        let y2 = cy + radius * theta2.sin();
        line(color, 1.0, [x1, y1, x2, y2], context.transform, graphics);
    }
}

pub fn draw_ellipse(
    color: Color,
    position: [u64; 2],
    size: [u64; 2],
    rotation: u64,
    c: &Context,
    g: &mut G2d,
) {
    let rotation_angle = (rotation as f64).to_radians();
    let transform = c
        .transform
        .trans(position[0] as f64, position[1] as f64)
        .rot_rad(rotation_angle);
    ellipse(
        color,
        [0.0, 0.0, size[0] as f64, size[1] as f64],
        transform,
        g,
    );
}

pub fn draw_detection_range(cx: f64, cy: f64, r: f64, color: [f32; 4], con: &Context, g: &mut G2d) {
    let segments = 12;
    let angle_offset = PI / 12.0;
    let mut points = Vec::with_capacity(segments);
    for i in 0..segments {
        let angle = angle_offset + i as f64 * (2.0 * PI / segments as f64);
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        points.push([x, y]);
    }
    for i in 0..segments {
        let next_index = (i + 1) % segments;
        line(
            color,
            0.5,
            [
                points[i][0],
                points[i][1],
                points[next_index][0],
                points[next_index][1],
            ],
            con.transform,
            g,
        );
        line(
            color,
            0.5,
            [points[i][0], points[i][1], cx, cy],
            con.transform,
            g,
        );
    }
}
