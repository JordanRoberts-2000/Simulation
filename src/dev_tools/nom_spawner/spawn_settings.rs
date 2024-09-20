use ::macroquad::prelude::*;
use std::rc::Rc;

use crate::utils::{selection::Selection, slider::Slider, toggle::ToggleSwitch};

use super::NomSpawner;

#[derive(Clone, PartialEq)]
pub enum NomLifeCycle {
    Baby,
    Adult,
    Old,
    Dead,
    Zombie,
}

#[derive(Clone, PartialEq)]
pub enum NomTwins {
    Off,
    On,
    Random,
}

impl NomSpawner {
    pub fn spawn_settings(&mut self) {
        self.create_random_spike_toggle();
        self.create_random_spike_slider();
        self.create_life_cycle_selector();
        self.create_twins_selector();
        self.set_toggle(350.0, 594.0);
        self.set_slider(100.0, 592.0, 140.0);
        self.set_life_cycle_selection(100.0, 565.0);
        self.set_twins_selection(100.0, 530.0);
    }

    pub fn draw_spawn_settings(&self) {
        if let Some(toggle) = &self.spike_random_toggle {
            toggle.draw();
        }
        if let Some(slider) = &self.spike_amount_slider {
            slider.draw();
        }
        if let Some(selection) = &self.life_cycle_selection {
            selection.draw();
        }
        if let Some(selection) = &self.twins_selection {
            selection.draw();
        }
        draw_text("Random", 266.0, 598.0, 18.0, WHITE);
        draw_text("Twins:", 20.0, 530.0, 20.0, WHITE);
        draw_text("Stage:", 20.0, 565.0, 20.0, WHITE);
        draw_text("Spikes:", 20.0, 600.0, 20.0, WHITE);
    }

    pub fn update_spawn_settings(&mut self) {
        if let Some(toggle) = &mut self.spike_random_toggle {
            toggle.update();
        }
        if let Some(slider) = &mut self.spike_amount_slider {
            slider.update();
            self.spike_amount = slider.get_index()
        }
        if let Some(selection) = &mut self.life_cycle_selection {
            selection.update();
            self.life_cycle = selection.get_selected();
        }
        if let Some(selection) = &mut self.twins_selection {
            selection.update();
            self.twins = selection.get_selected();
        }
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

    fn create_life_cycle_selector(&mut self) {
        let options = vec![
            ("Baby", NomLifeCycle::Baby),
            ("Adult", NomLifeCycle::Adult),
            ("Old", NomLifeCycle::Old),
            ("Dead", NomLifeCycle::Dead),
            ("Zombie", NomLifeCycle::Zombie),
        ];
        self.life_cycle_selection = Some(Selection::new(options, NomLifeCycle::Adult));
    }

    fn create_twins_selector(&mut self) {
        let options = vec![
            ("Off", NomTwins::Off),
            ("On", NomTwins::On),
            ("Random", NomTwins::Random),
        ];
        self.twins_selection = Some(Selection::new(options, NomTwins::Off));
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

    fn set_life_cycle_selection(&mut self, x: f32, y: f32) {
        if let Some(selection) = &mut self.life_cycle_selection {
            selection.set(x, y);
        }
    }

    fn set_twins_selection(&mut self, x: f32, y: f32) {
        if let Some(selection) = &mut self.twins_selection {
            selection.set(x, y);
        }
    }
}
