use std::{cell::RefCell, rc::Rc};

use crate::{simulation_state::SimulationState, utils::ui::button::Button};
use nom_spawner::NomSpawner;
use simulation_toggles::SimulationToggles;

pub mod command_buttons;
pub mod nom_spawner;
pub mod simulation_toggles;

pub struct SimulationTools {
    nom_spawner: NomSpawner,
    simulation_toggles: SimulationToggles,
    command_buttons: Vec<Button>,
}

impl SimulationTools {
    pub fn new(state: Rc<RefCell<SimulationState>>) -> Self {
        Self {
            nom_spawner: NomSpawner::new(state.clone()),
            simulation_toggles: SimulationToggles::new(),
            command_buttons: SimulationTools::create_command_buttons(state.clone()),
        }
    }

    pub fn update(&mut self, state: Rc<RefCell<SimulationState>>) {
        self.nom_spawner.update(state.clone());
        self.update_command_buttons();
        self.simulation_toggles.update(state);
    }

    pub fn draw(&self, state: Rc<RefCell<SimulationState>>) {
        self.nom_spawner.draw(state.clone());
        self.draw_command_buttons();
        self.simulation_toggles.draw(state);
    }
}
