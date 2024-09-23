use macroquad::prelude::*;

use crate::utils::draw::draw_rounded_rectangle;

const DEFAULT_PADDING: Vec2 = vec2(8.0, 6.0);
const DEFAULT_FONT_SIZE: u16 = 16;

pub struct Button {
    click_fn: Box<dyn FnMut()>,
    position: Vec2,
    text: String, // This will hold the button text
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
    hovered: bool,
}

impl Button {
    pub fn new(text: &str) -> Self {
        let text_dimensions = measure_text(text, None, DEFAULT_FONT_SIZE, 1.0);
        let (width, height) = (
            text_dimensions.width + DEFAULT_PADDING.x * 2.0,
            text_dimensions.height + DEFAULT_PADDING.y * 2.0,
        );
        Self {
            text: text.to_string(), // Assign the passed text here
            click_fn: Box::new(|| {}),
            position: vec2(0.0, 0.0),
            font_size: DEFAULT_FONT_SIZE,
            height,
            width,
            padding: DEFAULT_PADDING,
            color: BLACK,
            border_color: GRAY,
            text_color: WHITE,
            active_color: BLUE,
            clicked: false,
            click_timer: 0.0,
            hovered: false,
        }
    }

    pub fn draw(&self) {
        let border_width: f32 = 1.0;
        let corner_radius = self.height * 0.1;

        // Draw the outer border of the button
        draw_rounded_rectangle(
            self.position.x,
            self.position.y,
            self.width + self.padding.x,  // Consider padding in width
            self.height + self.padding.y, // Consider padding in height
            corner_radius,
            if self.hovered {
                BLUE
            } else {
                self.border_color
            },
        );

        // Draw the inner area of the button
        draw_rounded_rectangle(
            self.position.x + border_width,
            self.position.y + border_width,
            self.width + self.padding.x - border_width * 2.0,
            self.height + self.padding.y - border_width * 2.0,
            corner_radius,
            if self.clicked {
                self.active_color
            } else {
                self.color
            },
        );

        // Calculate the centered text with padding in mind
        let text_dimensions = measure_text(&self.text, None, self.font_size, 1.0);
        let text_x =
            self.position.x + self.padding.x / 2.0 + (self.width - text_dimensions.width) / 2.0;
        let text_y =
            self.position.y + self.padding.y / 2.0 + (self.height + text_dimensions.height) / 2.0
                - text_dimensions.height / 2.0;

        // Draw the text in the center of the button
        draw_text(
            &self.text,
            text_x,
            text_y + 4.0, // TEMP: offset in y <- look into
            self.font_size as f32,
            self.text_color,
        );
    }

    pub fn update(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        if self.contains(mouse_x, mouse_y, self.position) {
            self.hovered = true;
        } else {
            self.hovered = false;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if self.contains(mouse_x, mouse_y, self.position) {
                (self.click_fn)();
                self.clicked = true;
                self.click_timer = 0.2;
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

    fn contains(&self, mouse_x: f32, mouse_y: f32, pos: Vec2) -> bool {
        mouse_x >= pos.x
            && mouse_x <= pos.x + self.width + self.padding.x
            && mouse_y >= pos.y
            && mouse_y <= pos.y + self.height + self.padding.y
    }

    pub fn pos(&mut self, x: f32, y: f32) {
        self.position = vec2(x, y);
    }

    pub fn padding(&mut self, x: f32, y: f32) {
        self.padding = vec2(x, y);
    }

    pub fn on_click<F>(&mut self, click_fn: F)
    where
        F: FnMut() + 'static,
    {
        self.click_fn = Box::new(click_fn);
    }

    #[allow(dead_code)]
    pub fn click(&mut self) {
        (self.click_fn)();
    }
}
