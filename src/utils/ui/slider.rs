use macroquad::prelude::*;

use crate::utils::draw::draw_rounded_rectangle;

pub struct Slider {
    width: Option<f32>,
    height: f32,
    rounding: f32,
    handle_radius: f32,
    active_width: f32,
    length: u32,
    index: u32,
    position: Option<Vec2>,
    handle_position: Option<Vec2>,
    dragging: bool,
}

impl Slider {
    pub fn new(length: u32) -> Self {
        Self {
            width: None,
            height: 4.0,
            rounding: 2.0,
            handle_radius: 6.0,
            active_width: 0.0,
            length,
            index: 0,
            position: None,
            handle_position: None,
            dragging: false,
        }
    }

    pub fn draw(&self) {
        if let (Some(width), Some(slider_pos), Some(handle_pos)) =
            (self.width, self.position, self.handle_position)
        {
            draw_rounded_rectangle(
                slider_pos.x,
                slider_pos.y,
                width,
                self.height,
                self.rounding,
                WHITE,
            );
            draw_rounded_rectangle(
                slider_pos.x,
                slider_pos.y,
                self.active_width,
                self.height,
                self.rounding,
                BLUE,
            );
            let step_size = width / (self.length - 1) as f32;
            for i in 0..self.length {
                let tick_x = slider_pos.x + step_size * i as f32;
                let tick_y = slider_pos.y - 3.0;

                draw_rectangle(tick_x - 2.0, tick_y, 4.0, 10.0, WHITE);
            }
            draw_circle(handle_pos.x, handle_pos.y, self.handle_radius, WHITE);
        }
    }

    pub fn update(&mut self) {
        if let (Some(width), Some(pos)) = (self.width, self.position) {
            let mouse_pos = mouse_position();
            let mouse_x = mouse_pos.0;
            let mouse_y = mouse_pos.1;

            if is_mouse_button_down(MouseButton::Left) {
                if let Some(handle_pos) = self.handle_position {
                    let dist = ((mouse_x - handle_pos.x).powi(2)
                        + (mouse_y - handle_pos.y).powi(2))
                    .sqrt();
                    if dist < self.handle_radius || self.dragging {
                        self.dragging = true;
                        let clamped_x = mouse_x.clamp(pos.x, pos.x + width);
                        self.handle_position = Some(vec2(clamped_x, handle_pos.y));
                        self.active_width = clamped_x - pos.x;
                    }
                }
            } else if self.dragging {
                self.dragging = false;
                self.calculate_index();
                self.snap_slider_index();
            }
        }
    }

    fn snap_slider_index(&mut self) {
        if let (Some(width), Some(pos)) = (self.width, self.position) {
            let step_size = width / (self.length - 1) as f32;
            let x = pos.x + step_size * self.index as f32;
            let y = pos.y + self.height / 2.0;
            self.handle_position = Some(vec2(x, y));
            self.active_width = x - pos.x;
        }
    }

    fn calculate_index(&mut self) {
        if let (Some(width), Some(pos), Some(handle_pos)) =
            (self.width, self.position, self.handle_position)
        {
            let relative_x = handle_pos.x - pos.x;
            let step_size = width / (self.length - 1) as f32;
            let new_index = (relative_x / step_size).round() as u32;
            self.index = new_index.min(self.length - 1);
        }
    }

    pub fn set(&mut self, x: f32, y: f32, width: f32) {
        self.position = Some(vec2(x, y));
        self.width = Some(width);
        self.snap_slider_index();
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }
}
