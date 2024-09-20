use ::macroquad::prelude::*;
use std::rc::Rc;

use crate::utils::{slider::Slider, toggle::ToggleSwitch};

use super::NomSpawner;

impl NomSpawner {
    pub fn spawn_settings(&mut self) {
        self.create_random_spike_toggle();
        self.set_toggle(350.0, 594.0);
        self.create_random_spike_slider();
        self.set_slider(100.0, 592.0, 140.0);
    }

    pub fn draw_spawn_settings(&self) {
        if let Some(toggle) = &self.spike_random_toggle {
            toggle.draw();
        }
        if let Some(slider) = &self.spike_amount_slider {
            slider.draw();
        }
        draw_text("Random", 266.0, 598.0, 18.0, WHITE);
    }

    fn create_random_spike_toggle(&mut self) {
        let spike_random_clone_on = Rc::clone(&self.spike_random);
        let spike_random_clone_off = Rc::clone(&self.spike_random);
        self.spike_random_toggle = Some(ToggleSwitch::new(
            Box::new(move || {
                *spike_random_clone_on.borrow_mut() = true;
            }),
            Box::new(move || {
                *spike_random_clone_off.borrow_mut() = false;
            }),
        ));
    }

    fn create_random_spike_slider(&mut self) {
        self.spike_amount_slider = Some(Slider::new(5))
    }

    fn set_slider(&mut self, x: f32, y: f32, width: f32) {
        if let Some(slider) = &mut self.spike_amount_slider {
            slider.set(x, y, width);
        }
    }

    fn set_toggle(&mut self, x: f32, y: f32) {
        if let Some(toggle) = &mut self.spike_random_toggle {
            toggle.set(x, y);
        }
    }
}
