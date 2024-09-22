use macroquad::prelude::*;

use crate::dev_tools::command_line::commands::handle_clear;
use crate::utils::ui::button::Button;

use crate::dev_tools::SimulationTools;

impl SimulationTools {
    pub fn create_command_buttons() -> Vec<Button> {
        let mut command_buttons = Vec::new();

        let command_buttons_y: f32 = 674.0;
        let mut clear_button = Button::new("clear");
        clear_button.pos(28.0, command_buttons_y);
        // clear_button.on_click(|| handle_clear());
        command_buttons.push(clear_button);

        let mut freeze_button = Button::new("freeze");
        freeze_button.pos(100.0, command_buttons_y);
        freeze_button.on_click(|| println!("freeze"));
        command_buttons.push(freeze_button);

        let mut restart_button = Button::new("restart");
        restart_button.pos(179.0, command_buttons_y);
        restart_button.on_click(|| println!("restart"));
        command_buttons.push(restart_button);

        command_buttons
    }

    pub fn draw_command_buttons(&self) {
        for button in self.command_buttons.iter() {
            button.draw();
        }
    }

    pub fn update_command_buttons(&mut self) {
        for button in self.command_buttons.iter_mut() {
            button.update();
        }
    }
}
