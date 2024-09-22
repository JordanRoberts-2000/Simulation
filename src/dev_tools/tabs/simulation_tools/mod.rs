use crate::utils::ui::button::Button;
// use nom_spawner::NomSpawner;
use simulation_toggles::{SimulationToggles, ToggleKey};

pub mod command_buttons;
// pub mod nom_spawner;
pub mod simulation_toggles;

pub struct SimulationTools {
    // nom_spawner: NomSpawner,
    simulation_toggles: SimulationToggles,
    command_buttons: Vec<Button>,
}

impl SimulationTools {
    pub fn new() -> Self {
        Self {
            // nom_spawner: NomSpawner::new(),
            simulation_toggles: SimulationToggles::new(),
            command_buttons: SimulationTools::create_command_buttons(),
        }
    }

    pub fn update(&mut self) {
        // self.nom_spawner.update();
        self.update_command_buttons();
        self.simulation_toggles.update();
    }

    pub fn draw(&self) {
        // self.nom_spawner.draw();
        self.draw_command_buttons();
        self.simulation_toggles.draw();
    }

    pub fn get_quadtree_visual_value(&self) -> bool {
        self.simulation_toggles
            .get_toggle_value(ToggleKey::Quadtree)
    }
}
