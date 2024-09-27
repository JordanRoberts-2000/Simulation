use macroquad::prelude::*;

use crate::utils::ui::toggle::Toggle;

pub struct NomTools {
    toggle_all_button: Toggle,
}

impl NomTools {
    pub fn new() -> Self {
        Self {
            toggle_all_button: Toggle::new(350.0, 95.0),
        }
    }

    pub fn update(&mut self) {
        let mut temp = false;
        self.toggle_all_button.update(&mut temp);
    }

    pub fn draw(&self) {
        draw_text("Apply to all noms:", 20.0, 100.0, 24.0, WHITE);
        draw_line(20.0, 122.0, 380.0, 122.0, 1.0, GRAY);
        draw_text("Disable movement:", 20.0, 160.0, 24.0, WHITE);
        draw_text("Wandering_visuals:", 20.0, 200.0, 24.0, WHITE);
        draw_text("Orientation_visuals:", 20.0, 240.0, 24.0, WHITE);
        draw_text("Target_orientation_visuals:", 20.0, 280.0, 24.0, WHITE);
        draw_text("detection_radius_visuals:", 20.0, 320.0, 24.0, WHITE);
        let mut temp = false;
        self.toggle_all_button.draw(&mut temp);
    }
}
