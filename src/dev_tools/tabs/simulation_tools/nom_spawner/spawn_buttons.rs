use macroquad::prelude::*;

use super::NomSpawner;
use crate::utils::ui::button::Button;

impl NomSpawner {
    pub fn draw_spawn_buttons(&self) {
        for button in self.spawn_buttons.iter() {
            button.draw();
        }
    }

    pub fn update_spawn_buttons(&mut self) {
        for button in self.spawn_buttons.iter_mut() {
            button.update();
        }
    }

    pub fn create_spawn_buttons() -> Vec<Button> {
        let mut spawn_buttons = Vec::new();

        let spawn_buttons_y: f32 = 635.0;

        let mut spawn_one_button = Button::new("Spawn nom");
        spawn_one_button.pos(100.0, spawn_buttons_y);
        spawn_one_button.on_click(|| println!("add 1"));
        spawn_buttons.push(spawn_one_button);

        let mut spawn_five_button = Button::new("+ 5");
        spawn_five_button.pos(100.0, spawn_buttons_y);
        spawn_five_button.on_click(|| println!("add 5"));
        spawn_buttons.push(spawn_five_button);

        let mut spawn_ten_button = Button::new("+ 10");
        spawn_ten_button.pos(100.0, spawn_buttons_y);
        spawn_ten_button.on_click(|| println!("add 10"));
        spawn_buttons.push(spawn_ten_button);

        let mut spawn_twenty_button = Button::new("+ 20");
        spawn_twenty_button.pos(100.0, spawn_buttons_y);
        spawn_twenty_button.on_click(|| println!("add 20"));
        spawn_buttons.push(spawn_twenty_button);

        let mut spawn_fifty_button = Button::new("+ 50");
        spawn_fifty_button.pos(100.0, spawn_buttons_y);
        spawn_fifty_button.on_click(|| println!("add 50"));
        spawn_buttons.push(spawn_fifty_button);

        spawn_buttons
    }
}
