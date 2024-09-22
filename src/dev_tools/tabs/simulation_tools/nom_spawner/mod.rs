use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;
use spawn_settings::{NomLifeCycle, NomTwins};

use crate::nom::Nom;
use crate::utils::ui::button::Button;
use crate::utils::ui::selection::Selection;
use crate::utils::ui::slider::Slider;
use crate::utils::ui::toggle::ToggleSwitch;

mod nom_selector;
mod spawn_buttons;
mod spawn_settings;

pub struct NomSpawner {
    nom_selection_index: (usize, usize),
    display_noms: Vec<Nom>,
    spike_random: Rc<RefCell<bool>>,
    spike_random_toggle: Option<ToggleSwitch>,
    spike_amount: u32,
    spike_amount_slider: Slider,
    life_cycle: NomLifeCycle,
    life_cycle_selection: Selection<NomLifeCycle>,
    twins: NomTwins,
    twins_selection: Selection<NomTwins>,
    spawn_buttons: Vec<Button>,
}

impl NomSpawner {
    pub fn new() -> Self {
        let mut nom_spawner = Self {
            nom_selection_index: (0, 0),
            display_noms: NomSpawner::display_noms(),
            spike_random: Rc::new(RefCell::new(false)),
            spike_random_toggle: None,
            spike_amount: 0,
            spike_amount_slider: Slider::new(5),
            life_cycle: NomLifeCycle::Adult,
            life_cycle_selection: NomSpawner::create_life_cycle_selector(),
            twins: NomTwins::Off,
            twins_selection: NomSpawner::create_twins_selector(),
            spawn_buttons: NomSpawner::create_spawn_buttons(),
        };
        nom_spawner.configure_spawn_settings();
        nom_spawner
    }

    pub fn draw(&self) {
        self.draw_spawn_settings();
        self.draw_nom_selector();
        self.draw_spawn_buttons();
    }

    pub fn update(&mut self) {
        self.update_spawn_settings();
        self.update_nom_selector();
        self.update_spawn_buttons();
    }
}
