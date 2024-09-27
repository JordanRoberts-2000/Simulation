use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

use crate::{simulation_state::SimulationState, utils::ui::toggle::Toggle};

pub struct SimulationToggles {
    quadtree_visuals_toggle: Toggle,
    grid_visuals_toggle: Toggle,
}

impl SimulationToggles {
    pub fn new() -> Self {
        Self {
            quadtree_visuals_toggle: Toggle::new(350.0, 95.0),
            grid_visuals_toggle: Toggle::new(350.0, 135.0),
        }
    }

    pub fn draw(&self, state: Rc<RefCell<SimulationState>>) {
        draw_text("Quadgrid visuals:", 20.0, 100.0, 24.0, WHITE);
        draw_text("Spatial grid visuals:", 20.0, 140.0, 24.0, WHITE);
        draw_line(20.0, 162.0, 380.0, 162.0, 1.0, GRAY);
        self.grid_visuals_toggle
            .draw(&state.borrow().visuals().grid());
        self.quadtree_visuals_toggle
            .draw(&state.borrow().visuals().quadtree());
    }

    pub fn update(&mut self, state: Rc<RefCell<SimulationState>>) {
        self.quadtree_visuals_toggle
            .update(state.borrow_mut().visuals_mut().quadtree_mut());
        self.grid_visuals_toggle
            .update(state.borrow_mut().visuals_mut().grid_mut());
    }
}
