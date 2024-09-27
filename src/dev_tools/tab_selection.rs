use macroquad::prelude::*;

use crate::utils::draw::draw_rounded_rectangle;

use super::DevTools;

#[derive(PartialEq)]
pub enum Tabs {
    Simulation,
    Noms,
    Food,
    Data,
}

impl DevTools {
    pub fn draw_tab_selection(&self) {
        self.draw_sim_icon();
        self.draw_nom_icon();
        self.draw_food_icon();
        self.draw_data_icon();
        draw_line(20.0, 70.0, 380.0, 70.0, 1.0, GRAY);
    }

    pub fn update_tab_selection(&mut self) {
        let section_width = 100.0;
        let section_height = 60.0;
        let total_width = 400.0;

        let (mouse_x, mouse_y) = mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) {
            if mouse_y >= 0.0 && mouse_y <= section_height {
                if mouse_x >= 0.0 && mouse_x < section_width {
                    self.current_tab = Tabs::Simulation;
                } else if mouse_x >= section_width && mouse_x < section_width * 2.0 {
                    self.current_tab = Tabs::Noms;
                } else if mouse_x >= section_width * 2.0 && mouse_x < section_width * 3.0 {
                    self.current_tab = Tabs::Food;
                } else if mouse_x >= section_width * 3.0 && mouse_x < total_width {
                    self.current_tab = Tabs::Data;
                }
            }
        }
    }

    pub fn draw_sim_icon(&self) {
        let (size, border_w): (f32, f32) = (36.0, 1.5);
        let position = vec2(30.0, 15.0);
        let color = if self.current_tab == Tabs::Simulation {
            BLUE
        } else {
            WHITE
        };
        draw_rounded_rectangle(position.x, position.y, size, size, 5.0, color);
        draw_rounded_rectangle(
            position.x + border_w,
            position.y + border_w,
            size - border_w * 2.0,
            size - border_w * 2.0,
            5.0,
            BLACK,
        );
        draw_circle(position.x + 10.0, position.y + 10.0, 1.0, color);
        draw_circle(position.x + 16.0, position.y + 24.0, 2.0, color);
        draw_circle(position.x + 26.0, position.y + 20.0, 2.0, color);
        draw_circle(position.x + 28.0, position.y + 28.0, 1.0, color);
    }

    pub fn draw_nom_icon(&self) {
        let (size, border_w): (f32, f32) = (16.0, 2.0);
        let position = vec2(140.0, 35.0);
        let color = if self.current_tab == Tabs::Noms {
            BLUE
        } else {
            WHITE
        };
        draw_circle(position.x, position.y, size, color);
        draw_circle(position.x, position.y, size - border_w, BLACK);
        draw_circle(position.x + 3.0, position.y + 2.0, 2.0, color);
        draw_circle(position.x - 1.0, position.y - 2.0, 1.0, color);
        draw_circle(position.x - 4.0, position.y + 3.0, 1.0, color);
    }

    pub fn draw_food_icon(&self) {
        let position = vec2(250.0, 32.0);
        let color = if self.current_tab == Tabs::Food {
            BLUE
        } else {
            WHITE
        };
        draw_circle(position.x, position.y, 6.0, color);
        draw_circle(position.x + 10.0, position.y + 10.0, 4.0, color);
    }

    pub fn draw_data_icon(&self) {
        let (size, border_w): (f32, f32) = (26.0, 1.0);
        let position = vec2(335.0, 24.0);
        let bar_width: f32 = 8.0;
        let gap: f32 = 2.0;
        let bar_amount: u32 = 3;
        let heights = [0.33 * size, 0.66 * size, 1.0 * size];
        let color = if self.current_tab == Tabs::Data {
            BLUE
        } else {
            WHITE
        };

        for i in 0..bar_amount {
            let bar_height = heights[i as usize];
            let x_pos = position.x + i as f32 * (bar_width + gap);
            draw_rectangle(
                x_pos,
                position.y + (size - bar_height),
                bar_width,
                bar_height,
                color,
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
