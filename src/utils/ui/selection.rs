use macroquad::prelude::*;

pub struct Selection<T: PartialEq + Clone> {
    position: Option<Vec2>,
    options: Vec<(String, T)>,
    selected: T,
}

impl<T: PartialEq + Clone> Selection<T> {
    pub fn new(options: Vec<(&str, T)>, default: T) -> Self {
        Self {
            position: None,
            options: options
                .into_iter()
                .map(|(label, value)| (label.to_string(), value))
                .collect(),
            selected: default,
        }
    }

    pub fn update(&mut self) {
        if let Some(mut pos) = self.position {
            let mouse_pos = mouse_position();
            let mouse_x = mouse_pos.0;
            let mouse_y = mouse_pos.1;

            for (label, value) in &self.options {
                let text_width = measure_text(label, None, 20, 1.0).width;
                let padding = 2.0;

                if is_mouse_button_pressed(MouseButton::Left)
                    && mouse_x >= pos.x
                    && mouse_x <= pos.x + text_width
                    && mouse_y >= pos.y - (20.0 + padding)
                    && mouse_y <= pos.y + padding
                {
                    self.selected = value.clone();
                }
                pos.x += text_width + 20.0;
            }
        }
    }

    pub fn draw(&self) {
        if let Some(mut pos) = self.position {
            for (label, value) in &self.options {
                let color = if *value == self.selected { BLUE } else { WHITE };
                draw_text(label, pos.x, pos.y, 20.0, color);
                let text_width = measure_text(label, None, 20, 1.0).width;
                pos.x += text_width + 20.0;
            }
        }
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.position = Some(vec2(x, y));
    }

    pub fn get_selected(&self) -> T {
        self.selected.clone()
    }
}
