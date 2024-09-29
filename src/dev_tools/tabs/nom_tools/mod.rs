use macroquad::prelude::*;
use nom_buttons::NomButtons;
use nom_toggles::NomToggles;
use std::{cell::RefCell, rc::Rc};

use crate::{nom::Nom, simulation_state::SimulationState};

mod nom_buttons;
mod nom_toggles;

pub struct NomTools {
    nom_toggles: NomToggles,
    nom_buttons: NomButtons,
}

impl NomTools {
    pub fn new() -> Self {
        Self {
            nom_toggles: NomToggles::new(),
            nom_buttons: NomButtons::new(),
        }
    }

    pub fn update(&mut self, state: Rc<RefCell<SimulationState>>) {
        self.nom_toggles.update(state);
        self.nom_buttons.update();
    }

    pub fn draw(&self, state: Rc<RefCell<SimulationState>>) {
        self.nom_toggles.draw(state.clone());
        self.nom_buttons.draw();

        if let Some(selected_nom) = state.borrow().selected_nom.borrow().as_ref() {
            self.draw_nom_stats(selected_nom.clone());
        } else {
            draw_text("Select nom to see stats", 70.0, 430.0, 24.0, WHITE);
        }
    }

    pub fn draw_nom_stats(&self, selected_nom: Rc<RefCell<Nom>>) {
        let nom = selected_nom.borrow();

        let stats = vec![
            format!("Size: {:.0}", nom.size),
            format!(
                "Target Position: ({:.0}, {:.0})",
                nom.target_position.x, nom.target_position.y
            ),
            format!("Orientation: {:.0}", nom.orientation.to_degrees()),
            format!(
                "Target Orientation: {:.0}",
                nom.target_orientation.to_degrees()
            ),
            format!("Current Speed: {:.0}", nom.current_speed),
            format!("Max Speed: {:.0}", nom.max_speed),
            format!("Acceleration: {:.0}", nom.acceleration),
            format!("Turning Speed: {:.0}", nom.turning_speed.to_degrees()),
        ];

        // Starting position for the text
        let mut y_position = 420.0;

        for stat in stats {
            draw_text(&stat, 20.0, y_position, 20.0, WHITE);
            y_position += 30.0; // Move down by 40 pixels for each line
        }
    }
}
