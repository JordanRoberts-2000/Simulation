use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;
use spawn_settings::{NomLifeCycle, NomTwins};

use crate::nom::{Nom, NomVariant};
use crate::simulation_state::SimulationState;
use crate::utils::ui::button::Button;
use crate::utils::ui::selection::Selection;
use crate::utils::ui::slider::Slider;
use crate::utils::ui::toggle::Toggle;

mod nom_selector;
mod spawn_buttons;
mod spawn_settings;

pub struct NomSpawner {
    nom_variant_selected: Rc<RefCell<NomVariant>>,
    display_noms: Vec<Nom>,
    spike_random_toggle: Toggle,
    spike_amount: u32,
    spike_amount_slider: Slider,
    life_cycle: NomLifeCycle,
    life_cycle_selection: Selection<NomLifeCycle>,
    twins: NomTwins,
    twins_selection: Selection<NomTwins>,
    spawn_buttons: Vec<Button>,
}

impl NomSpawner {
    pub fn new(state: Rc<RefCell<SimulationState>>) -> Self {
        let selected_variant = Rc::new(RefCell::new(NomVariant::Default));
        let mut nom_spawner = Self {
            nom_variant_selected: selected_variant.clone(),
            display_noms: NomSpawner::create_display_noms(),
            spike_random_toggle: Toggle::new(350.0, 594.0),
            spike_amount: 0,
            spike_amount_slider: Slider::new(5),
            life_cycle: NomLifeCycle::Adult,
            life_cycle_selection: NomSpawner::create_life_cycle_selector(),
            twins: NomTwins::Off,
            twins_selection: NomSpawner::create_twins_selector(),
            spawn_buttons: NomSpawner::create_spawn_buttons(
                state.borrow_mut().noms.clone(),
                selected_variant,
            ),
        };
        nom_spawner.configure_spawn_settings();
        nom_spawner
    }

    pub fn draw(&self, state: Rc<RefCell<SimulationState>>) {
        self.draw_spawn_settings(state);
        self.draw_nom_selector();
        self.draw_spawn_buttons();
    }

    pub fn update(&mut self, state: Rc<RefCell<SimulationState>>) {
        self.update_spawn_settings(state);
        self.update_nom_selector();
        self.update_spawn_buttons();
    }
}
