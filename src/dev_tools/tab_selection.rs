use macroquad::prelude::*;

use crate::utils::draw::draw_rounded_rectangle;

use super::DevTools;

impl DevTools {
    pub fn draw_tab_selection(&self) {
        {
            let (size, border_w): (f32, f32) = (36.0, 1.5);
            let position = vec2(30.0, 15.0);
            draw_rounded_rectangle(position.x, position.y, size, size, 5.0, WHITE);
            draw_rounded_rectangle(
                position.x + border_w,
                position.y + border_w,
                size - border_w * 2.0,
                size - border_w * 2.0,
                5.0,
                BLACK,
            );
            draw_circle(position.x + 10.0, position.y + 10.0, 1.0, WHITE);
            draw_circle(position.x + 16.0, position.y + 24.0, 2.0, WHITE);
            draw_circle(position.x + 26.0, position.y + 20.0, 2.0, WHITE);
            draw_circle(position.x + 28.0, position.y + 28.0, 1.0, WHITE);
        }
        {
            let (size, border_w): (f32, f32) = (16.0, 2.0);
            let position = vec2(140.0, 35.0);
            draw_circle(position.x, position.y, size, WHITE);
            draw_circle(position.x, position.y, size - border_w, BLACK);
            draw_circle(position.x + 3.0, position.y + 2.0, 2.0, WHITE);
            draw_circle(position.x - 1.0, position.y - 2.0, 1.0, WHITE);
            draw_circle(position.x - 4.0, position.y + 3.0, 1.0, WHITE);
        }
        {
            let position = vec2(250.0, 32.0);
            draw_circle(position.x, position.y, 6.0, WHITE);
            draw_circle(position.x + 10.0, position.y + 10.0, 4.0, WHITE);
        }
        {
            let (size, border_w): (f32, f32) = (26.0, 1.0);
            let position = vec2(335.0, 24.0);
            let bar_width: f32 = 8.0;
            let gap: f32 = 2.0;
            let bar_amount: u32 = 3;
            let heights = [0.33 * size, 0.66 * size, 1.0 * size];

            for i in 0..bar_amount {
                let bar_height = heights[i as usize];
                let x_pos = position.x + i as f32 * (bar_width + gap);
                draw_rectangle(
                    x_pos,
                    position.y + (size - bar_height),
                    bar_width,
                    bar_height,
                    WHITE,
                );
                draw_rectangle(
                    x_pos + border_w,
                    position.y + (size - bar_height) + border_w,
                    bar_width - border_w * 2.0,
                    bar_height - border_w * 2.0,
                    BLACK,
                );
            }
        }
    }
}
