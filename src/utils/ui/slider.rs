use macroquad::prelude::*;

use crate::utils::draw::draw_rounded_rectangle;

pub struct Slider {
    width: f32,
    height: f32,
    rounding: f32,
    handle_radius: f32,
    active_width: f32,
    length: u32,
    position: Vec2,
    handle_position: Vec2,
    dragging: bool,
}

impl Slider {
    pub fn new(x: f32, y: f32, length: u32, index: u32) -> Self {
        let width = 140.0;
        let step_size = width / (length - 1) as f32;
        let handle_position = vec2(x + step_size * index as f32, y + 2.0);

        Self {
            width,
            height: 4.0,
            rounding: 2.0,
            handle_radius: 6.0,
            active_width: step_size * index as f32,
            length,
            position: vec2(x, y),
            handle_position,
            dragging: false,
        }
    }

    pub fn draw(&self, index: u32) {
        let step_size = self.width / (self.length - 1) as f32;
        let active_width = step_size * index as f32;

        draw_rounded_rectangle(
            self.position.x,
            self.position.y,
            self.width,
            self.height,
            self.rounding,
            WHITE,
        );

        draw_rounded_rectangle(
            self.position.x,
            self.position.y,
            active_width,
            self.height,
            self.rounding,
            BLUE,
        );

        for i in 0..self.length {
            let tick_x = self.position.x + step_size * i as f32;
            let tick_y = self.position.y - 3.0;
            draw_rectangle(tick_x - 2.0, tick_y, 4.0, 10.0, WHITE);
        }

        draw_circle(
            self.position.x + active_width,
            self.handle_position.y,
            self.handle_radius,
            WHITE,
        );
    }

    pub fn update(&mut self, index: &mut u32) {
        let mouse_pos = mouse_position();
        let mouse_x = mouse_pos.0;
        let mouse_y = mouse_pos.1;

        if is_mouse_button_down(MouseButton::Left) {
            let dist = ((mouse_x - self.position.x - self.active_width).powi(2)
                + (mouse_y - self.handle_position.y).powi(2))
            .sqrt();

            if dist < self.handle_radius || self.dragging {
                self.dragging = true;
                let clamped_x = mouse_x.clamp(self.position.x, self.position.x + self.width);
                self.handle_position.x = clamped_x;
                self.active_width = clamped_x - self.position.x;
            }
        } else if self.dragging {
            self.dragging = false;
            self.calculate_index(index);
            self.snap_slider_index(*index);
        }
    }

    fn snap_slider_index(&mut self, index: u32) {
        let step_size = self.width / (self.length - 1) as f32;
        self.handle_position.x = self.position.x + step_size * index as f32;
        self.active_width = self.handle_position.x - self.position.x;
    }

    fn calculate_index(&mut self, index: &mut u32) {
        let relative_x = self.handle_position.x - self.position.x;
        let step_size = self.width / (self.length - 1) as f32;
        let new_index = (relative_x / step_size).round() as u32;
        *index = new_index.min(self.length - 1);
    }
}
