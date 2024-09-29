use macroquad::prelude::*;
use std::{cell::RefCell, rc::Rc};

use crate::{simulation_state::SimulationState, utils::ui::toggle::Toggle};

pub struct NomToggles {
    toggle_all_toggle: Toggle,
    disable_movement_toggle: Toggle,
    wandering_toggle: Toggle,
    orientation_toggle: Toggle,
    target_orientation_toggle: Toggle,
    detection_radius_toggle: Toggle,
    collisions_toggle: Toggle,
}

impl NomToggles {
    pub fn new() -> Self {
        Self {
            toggle_all_toggle: Toggle::new(350.0, 95.0),
            disable_movement_toggle: Toggle::new(350.0, 155.0),
            wandering_toggle: Toggle::new(350.0, 195.0),
            orientation_toggle: Toggle::new(350.0, 235.0),
            target_orientation_toggle: Toggle::new(350.0, 275.0),
            detection_radius_toggle: Toggle::new(350.0, 315.0),
            collisions_toggle: Toggle::new(350.0, 355.0),
        }
    }
    pub fn update(&mut self, state: Rc<RefCell<SimulationState>>) {
        self.toggle_all_toggle
            .update(&mut state.borrow_mut().devtools.apply_to_all);
        self.disable_movement_toggle
            .update(&mut state.borrow_mut().devtools.disable_movement);
        self.wandering_toggle
            .update(&mut state.borrow_mut().devtools.visuals.nom_wandering);
        self.orientation_toggle
            .update(&mut state.borrow_mut().devtools.visuals.nom_orientation);
        self.target_orientation_toggle
            .update(&mut state.borrow_mut().devtools.visuals.nom_target_orientation);
        self.detection_radius_toggle
            .update(&mut state.borrow_mut().devtools.visuals.nom_detection_radius);
        self.collisions_toggle
            .update(&mut state.borrow_mut().devtools.visuals.collisions);
    }

    pub fn draw(&self, state: Rc<RefCell<SimulationState>>) {
        draw_text("Apply to all noms:", 20.0, 100.0, 24.0, WHITE);
        draw_line(20.0, 122.0, 380.0, 122.0, 1.0, GRAY);
        draw_text("Disable movement:", 20.0, 160.0, 24.0, WHITE);
        draw_text("Wandering visuals:", 20.0, 200.0, 24.0, WHITE);
        draw_text("Orientation visuals:", 20.0, 240.0, 24.0, WHITE);
        draw_text("Target orientation visuals:", 20.0, 280.0, 24.0, WHITE);
        draw_text("Detection radius visuals:", 20.0, 320.0, 24.0, WHITE);
        draw_text("Collision visuals:", 20.0, 360.0, 24.0, WHITE);
        draw_line(20.0, 388.0, 380.0, 388.0, 1.0, GRAY);
        self.toggle_all_toggle
            .draw(&state.borrow().devtools.apply_to_all);
        self.disable_movement_toggle
            .draw(&state.borrow().devtools.disable_movement);
        self.wandering_toggle
            .draw(&state.borrow().devtools.visuals.nom_wandering);
        self.orientation_toggle
            .draw(&state.borrow().devtools.visuals.nom_orientation);
        self.target_orientation_toggle
            .draw(&state.borrow().devtools.visuals.nom_target_orientation);
        self.detection_radius_toggle
            .draw(&state.borrow().devtools.visuals.nom_detection_radius);
        self.collisions_toggle
            .draw(&state.borrow().devtools.visuals.collisions);
    }
}
