use crate::utils::toggle::ToggleSwitch;
use macroquad::prelude::*;

pub struct Toggles {
    toggles: Vec<ToggleSwitch>,
}

impl Toggles {
    pub fn new() -> Self {
        Self {
            toggles: Vec::new(),
        }
    }

    pub fn handle_inputs(&mut self) {
        for toggle in &mut self.toggles {
            toggle.check_click()
        }
    }

    pub fn draw(&self) {
        for toggle in &self.toggles {
            toggle.draw();
        }
    }

    pub fn add_toggle(
        &mut self,
        position: Vec2,
        on_toggle_on: Box<dyn Fn()>,
        on_toggle_off: Box<dyn Fn()>,
    ) {
        self.toggles
            .push(ToggleSwitch::new(position, on_toggle_on, on_toggle_off));
    }
}
