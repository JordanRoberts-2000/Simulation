use crate::utils::toggle::ToggleSwitch;
use macroquad::prelude::*;

pub struct Toggles {
    toggles: Vec<ToggleSwitch>,
}

impl Toggles {
    pub fn new() -> Self {
        Self {
            toggles: vec![
                ToggleSwitch::new(
                    vec2(350., 25.),
                    Box::new(|| println!("Quadtree turned on")),
                    Box::new(|| println!("Quadtree turned of")),
                ),
                ToggleSwitch::new(
                    vec2(350., 75.),
                    Box::new(|| println!("NomVisuals turned on")),
                    Box::new(|| println!("NomVisuals turned of")),
                ),
            ],
        }
    }

    pub fn update(&mut self) {
        for toggle in &mut self.toggles {
            toggle.check_click()
        }
    }

    pub fn draw(&self) {
        for toggle in &self.toggles {
            toggle.draw();
        }
    }
}
