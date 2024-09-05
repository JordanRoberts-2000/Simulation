use macroquad::prelude::*;

pub struct DevTools {
    pub devtools_active: bool,
    command_line: String,
    testing_visuals: bool,
}

impl DevTools {
    pub fn new() -> Self {
        Self {
            devtools_active: false,
            command_line: String::new(),
            testing_visuals: false,
        }
    }

    fn submit(&mut self) {
        println!("Input submitted: {}", self.command_line);
        self.command_line.clear();
    }

    pub fn handle_inputs(&mut self) {
        if is_key_pressed(KeyCode::LeftShift) {
            self.devtools_active = !self.devtools_active;
        }
        if self.devtools_active {
            // Handle special keys
            if is_key_pressed(KeyCode::Enter) {
                self.submit();
            } else if is_key_pressed(KeyCode::Backspace) {
                self.command_line.pop();
            }

            // Capture regular character input
            while let Some(ch) = get_char_pressed() {
                self.command_line.push(ch);
            }
        }
    }

    pub fn draw(&self) {
        if !self.devtools_active {
            return;
        };
        draw_rectangle(10.0, screen_height() - 40.0, 200.0, 30.0, GRAY);
        draw_text(&self.command_line, 15.0, screen_height() - 20.0, 20.0, RED);

        if self.command_line.is_empty() {
            draw_text("Commands:", 15.0, screen_height() - 20.0, 20.0, LIGHTGRAY);
        }
    }
}
