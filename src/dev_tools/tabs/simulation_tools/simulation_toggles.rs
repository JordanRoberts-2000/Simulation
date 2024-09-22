use macroquad::prelude::*;

use crate::utils::ui::toggle::Toggle;

pub enum ToggleKey {
    Quadtree,
    Grid,
}

pub struct SimulationToggles {
    quadtree_visuals_toggle: Toggle,
    grid_visuals_toggle: Toggle,
}

impl SimulationToggles {
    pub fn new() -> Self {
        Self {
            quadtree_visuals_toggle: Toggle::new(350.0, 95.0),
            grid_visuals_toggle: Toggle::new(350.0, 135.0),
        }
    }

    pub fn draw(&self) {
        draw_text("Quadgrid visuals", 20.0, 100.0, 24.0, WHITE);
        draw_text("Spatial grid visuals", 20.0, 140.0, 24.0, WHITE);
        draw_line(20.0, 162.0, 380.0, 162.0, 1.0, GRAY);
        draw_line(20.0, 70.0, 380.0, 70.0, 1.0, GRAY);
        self.grid_visuals_toggle.draw();
        self.quadtree_visuals_toggle.draw();
    }

    pub fn update(&mut self) {
        self.quadtree_visuals_toggle.update();
        self.grid_visuals_toggle.update();
    }

    pub fn get_toggle_value(&self, key: ToggleKey) -> bool {
        match key {
            ToggleKey::Quadtree => self.quadtree_visuals_toggle.get_value(),
            ToggleKey::Grid => self.grid_visuals_toggle.get_value(),
        }
    }
}
