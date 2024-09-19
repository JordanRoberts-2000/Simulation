use macroquad::prelude::*;

use super::draw::draw_rounded_rectangle;

pub struct Button {
    on_click: Box<dyn Fn()>,
    position: Option<Vec2>,
    text: String,
    font_size: u16,
    height: f32,
    padding: Vec2,
    width: f32,
    color: Color,
    border_color: Color,
    text_color: Color,
    active_color: Color,
    clicked: bool,
    click_timer: f32,
}

impl Button {
    pub fn new(
        on_click: Box<dyn Fn()>,
        font_size: u16,
        padding: Vec2,
        color: Color,
        border_color: Color,
        text_color: Color,
        active_color: Color,
    ) -> Self {
        Self {
            text: String::new(),
            on_click,
            position: None,
            font_size,
            height: 0.0,
            width: 0.0,
            padding,
            color,
            border_color,
            text_color,
            active_color,
            clicked: false,
            click_timer: 0.0,
        }
    }
    pub fn draw(&self) {
        if let Some(pos) = self.position {
            let border_width: f32 = 1.0;
            let corner_radius = self.height * 0.1;
            draw_rounded_rectangle(
                pos.x,
                pos.y,
                self.width,
                self.height,
                corner_radius,
                self.border_color,
            );
            draw_rounded_rectangle(
                pos.x + border_width,
                pos.y + border_width,
                self.width - border_width * 2.0,
                self.height - border_width * 2.0,
                corner_radius,
                if self.clicked {
                    self.active_color
                } else {
                    self.color
                },
            );
            let text_dimensions = measure_text(&self.text, None, self.font_size, 1.0);

            // Calculate the centered position for the text
            let text_x = pos.x + (self.width - text_dimensions.width) / 2.0;
            let text_y =
                pos.y + (self.height + text_dimensions.height) / 2.0 - text_dimensions.height / 2.0;
            draw_text(
                &self.text,
                text_x,
                text_y + 4.0,
                self.font_size as f32,
                self.text_color,
            );
        }
    }

    pub fn update(&mut self) {
        if let Some(pos) = self.position {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();
                if self.contains(mouse_x, mouse_y, pos) {
                    (self.on_click)();
                    self.clicked = true;
                    self.click_timer = 0.2;
                }
            }
        }
        if self.clicked {
            let delta_time = get_frame_time();
            self.click_timer -= delta_time;
            if self.click_timer <= 0.0 {
                self.clicked = false;
                self.click_timer = 0.0;
            }
        }
    }

    pub fn set(&mut self, x: f32, y: f32, text: &str) {
        self.position = Some(vec2(x, y));
        self.text = text.to_string();
        let text_dimensions = measure_text(text, None, self.font_size, 1.0);
        let (width, height) = (
            text_dimensions.width + self.padding.x * 2.0,
            text_dimensions.height + self.padding.y * 2.0,
        );
        self.width = width;
        self.height = height;
    }

    fn contains(&self, mouse_x: f32, mouse_y: f32, pos: Vec2) -> bool {
        mouse_x >= pos.x
            && mouse_x <= pos.x + self.width
            && mouse_y >= pos.y
            && mouse_y <= pos.y + self.height
    }
}
