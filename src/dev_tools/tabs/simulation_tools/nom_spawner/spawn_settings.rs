use ::macroquad::prelude::*;
use std::{cell::RefCell, rc::Rc};

use crate::{simulation_state::SimulationState, utils::ui::selection::Selection};

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
    pub fn configure_spawn_settings(&mut self) {
        self.spike_amount_slider.set(106.0, 592.0, 140.0);
        self.life_cycle_selection.set(100.0, 565.0);
        self.twins_selection.set(100.0, 530.0);
    }

    pub fn draw_spawn_settings(&self, state: Rc<RefCell<SimulationState>>) {
        self.spike_random_toggle
            .draw(&state.borrow().behaviors().spike_mutation());
        self.spike_amount_slider.draw();
        self.life_cycle_selection.draw();
        self.twins_selection.draw();
        draw_text("Random", 266.0, 598.0, 18.0, WHITE);
        draw_text("Twins:", 20.0, 530.0, 22.0, WHITE);
        draw_text("Stage:", 20.0, 565.0, 22.0, WHITE);
        draw_text("Spikes:", 20.0, 600.0, 22.0, WHITE);
    }

    pub fn update_spawn_settings(&mut self, state: Rc<RefCell<SimulationState>>) {
        self.spike_random_toggle
            .update(state.borrow_mut().behaviors_mut().spike_mutation_mut());
        self.spike_amount_slider.update();
        self.spike_amount = self.spike_amount_slider.get_index();
        self.life_cycle_selection.update();
        self.life_cycle = self.life_cycle_selection.get_selected();
        self.twins_selection.update();
        self.twins = self.twins_selection.get_selected();
    }

    pub fn create_life_cycle_selector() -> Selection<NomLifeCycle> {
        let options = vec![
            ("Baby", NomLifeCycle::Baby),
            ("Adult", NomLifeCycle::Adult),
            ("Old", NomLifeCycle::Old),
            ("Dead", NomLifeCycle::Dead),
            ("Zombie", NomLifeCycle::Zombie),
        ];
        Selection::new(options, NomLifeCycle::Adult)
    }

    pub fn create_twins_selector() -> Selection<NomTwins> {
        let options = vec![
            ("Off", NomTwins::Off),
            ("On", NomTwins::On),
            ("Random", NomTwins::Random),
        ];
        Selection::new(options, NomTwins::Off)
    }
}
