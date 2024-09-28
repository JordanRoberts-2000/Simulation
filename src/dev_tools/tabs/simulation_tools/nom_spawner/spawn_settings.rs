use ::macroquad::prelude::*;
use std::{cell::RefCell, rc::Rc};

use crate::{
    simulation_state::SimulationState,
    utils::ui::{selection::Selection, slider::Slider, toggle::Toggle},
};

#[derive(Clone, PartialEq)]
pub enum NomLifeCycle {
    Baby,
    Adult,
    Old,
    Dead,
}

#[derive(Clone, PartialEq)]
pub enum NomTwins {
    Off,
    On,
    Random,
}

pub struct NomSpawnerSettings {
    selection_twins: Selection<NomTwins>,
    slider_spike_amount: Slider,
    toggle_random_spikes: Toggle,
    selection_life_cycle: Selection<NomLifeCycle>,
}

impl NomSpawnerSettings {
    pub fn new() -> Self {
        Self {
            selection_twins: create_twins_selector(),
            slider_spike_amount: Slider::new(106.0, 592.0, 4),
            toggle_random_spikes: Toggle::new(350.0, 594.0),
            selection_life_cycle: create_life_cycle_selector(),
        }
    }

    pub fn draw(&self, state: Rc<RefCell<SimulationState>>) {
        draw_text("Random", 266.0, 598.0, 18.0, WHITE);
        draw_text("Twins:", 20.0, 530.0, 22.0, WHITE);
        draw_text("Stage:", 20.0, 565.0, 22.0, WHITE);
        draw_text("Spikes:", 20.0, 600.0, 22.0, WHITE);
        self.selection_twins
            .draw(&state.borrow().devtools.twin_mutation);
        self.slider_spike_amount
            .draw(&state.borrow().devtools.spike_amount);
        self.toggle_random_spikes
            .draw(&state.borrow().devtools.random_spike_mutation);
        self.selection_life_cycle
            .draw(&state.borrow().devtools.nom_life_cycle);
    }

    pub fn update(&mut self, state: Rc<RefCell<SimulationState>>) {
        self.selection_twins
            .update(&mut state.borrow_mut().devtools.twin_mutation);
        self.slider_spike_amount
            .update(&mut state.borrow_mut().devtools.spike_amount);
        self.toggle_random_spikes
            .update(&mut state.borrow_mut().devtools.random_spike_mutation);
        self.selection_life_cycle
            .update(&mut state.borrow_mut().devtools.nom_life_cycle);
    }
}

fn create_life_cycle_selector() -> Selection<NomLifeCycle> {
    let options = vec![
        ("Baby", NomLifeCycle::Baby),
        ("Adult", NomLifeCycle::Adult),
        ("Old", NomLifeCycle::Old),
        ("Dead", NomLifeCycle::Dead),
    ];
    Selection::new(100.0, 565.0, options)
}

fn create_twins_selector() -> Selection<NomTwins> {
    let options = vec![
        ("Off", NomTwins::Off),
        ("On", NomTwins::On),
        ("Random", NomTwins::Random),
    ];
    Selection::new(100.0, 530.0, options)
}
