use std::{cell::RefCell, rc::Rc};

use spawn_settings::NomSpawnerSettings;

use crate::nom::{Nom, NomVariant};
use crate::simulation_state::SimulationState;
use crate::utils::ui::button::Button;

mod nom_selector;
mod spawn_buttons;
pub mod spawn_settings;

pub struct NomSpawner {
    nom_variant_selected: Rc<RefCell<NomVariant>>,
    display_noms: Vec<Nom>,
    spawn_settings: NomSpawnerSettings,
    spawn_buttons: Vec<Button>,
}

impl NomSpawner {
    pub fn new(state: Rc<RefCell<SimulationState>>) -> Self {
        let selected_variant = Rc::new(RefCell::new(NomVariant::Default));
        Self {
            nom_variant_selected: selected_variant.clone(),
            display_noms: NomSpawner::create_display_noms(),
            spawn_settings: NomSpawnerSettings::new(),
            spawn_buttons: NomSpawner::create_spawn_buttons(state, selected_variant),
        }
    }

    pub fn draw(&self, state: Rc<RefCell<SimulationState>>) {
        self.spawn_settings.draw(state);
        self.draw_nom_selector();
        self.draw_spawn_buttons();
    }

    pub fn update(&mut self, state: Rc<RefCell<SimulationState>>) {
        self.spawn_settings.update(state);
        self.update_nom_selector();
        self.update_spawn_buttons();
    }
}
