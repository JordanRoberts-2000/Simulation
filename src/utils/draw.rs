use macroquad::prelude::*;

pub fn draw_rounded_rectangle(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color) {
    // Draw the 4 corner circles
    draw_circle(x + radius, y + radius, radius, color); // Top-left
    draw_circle(x + width - radius, y + radius, radius, color); // Top-right
    draw_circle(x + radius, y + height - radius, radius, color); // Bottom-left
    draw_circle(x + width - radius, y + height - radius, radius, color); // Bottom-right

    // Draw the 4 edge rectangles
    draw_rectangle(x + radius, y, width - 2.0 * radius, radius, color); // Top edge
    draw_rectangle(
        x + radius,
        y + height - radius,
        width - 2.0 * radius,
        radius,
        color,
    ); // Bottom edge
    draw_rectangle(x, y + radius, radius, height - 2.0 * radius, color); // Left edge
    draw_rectangle(
        x + width - radius,
        y + radius,
        radius,
        height - 2.0 * radius,
        color,
    ); // Right edge

    // Draw the center rectangle
    draw_rectangle(
        x + radius,
        y + radius,
        width - 2.0 * radius,
        height - 2.0 * radius,
        color,
    ); // Center
}
