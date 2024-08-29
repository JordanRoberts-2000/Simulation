use piston_window::types::Color;
use piston_window::*;
use piston_window::{ellipse, line, Context, G2d};
use std::f64::consts::PI;

pub fn draw_line(
    color: Color,
    coord_1: [u64; 2],
    coord_2: [u64; 2],
    thickness: f64,
    c: &Context,
    g: &mut G2d,
) {
    let (x1, y1) = (coord_1[0] as f64, coord_1[1] as f64);
    let (x2, y2) = (coord_2[0] as f64, coord_2[1] as f64);
    line(color, thickness, [x1, y1, x2, y2], c.transform, g);
}

pub fn draw_circle(color: Color, position: [u64; 2], size: u64, c: &Context, g: &mut G2d) {
    let x = position[0]
        .checked_sub(size / 2) // position parameter now circle center
        .expect("draw_circle: underflow detected on X coordinate");
    let y = position[1]
        .checked_sub(size / 2)
        .expect("draw_circle: underflow detected on X coordinate");
    ellipse(
        color,
        [x as f64, y as f64, size as f64, size as f64],
        c.transform,
        g,
    );
}

pub fn draw_hollow_circle(
    color: Color,
    position: [u64; 2],
    size: u64,
    thickness: f64,
    segments: usize,
    c: &Context,
    g: &mut G2d,
) {
    let (cx, cy) = (position[0] as f64, position[1] as f64);
    let step = 2.0 * PI / segments as f64;
    for i in 0..segments {
        let theta1 = i as f64 * step;
        let theta2 = (i + 1) as f64 * step;
        let x1 = cx + ((size / 2) as f64) * theta1.cos();
        let y1 = cy + ((size / 2) as f64) * theta1.sin();
        let x2 = cx + ((size / 2) as f64) * theta2.cos();
        let y2 = cy + ((size / 2) as f64) * theta2.sin();
        line(color, thickness, [x1, y1, x2, y2], c.transform, g);
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
