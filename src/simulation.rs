use std::cell::RefCell;
use std::rc::Rc;

use macroquad::prelude::*;
use miniquad::window::quit;

use crate::dev_tools::DevTools;
use crate::entity_stats::EntityStats;
use crate::nom::{Nom, NomVariant};
use crate::quadtree::Quadtree;
use crate::simulation_state::SimulationState;

pub struct Simulation {
    state: Rc<RefCell<SimulationState>>,
    dev_tools: DevTools,
    environment_stats: bool,
    entity_stats: EntityStats,
}

impl Simulation {
    pub fn new() -> Simulation {
        let noms = Rc::new(RefCell::new(vec![Rc::new(RefCell::new(Nom::new(
            vec2(350.0, 350.0),
            NomVariant::Default,
        )))]));

        // Insert noms into the quadtree
        let quadtree = Rc::new(RefCell::new(Quadtree::new()));
        for nom in noms.borrow().iter() {
            quadtree.borrow_mut().insert(Rc::clone(nom));
        }

        let state = Rc::new(RefCell::new(SimulationState::new(noms, quadtree)));
        let entity_stats = {
            let state_borrow = state.borrow();
            EntityStats::new(state_borrow.selected_nom.clone())
        };
        let sim = Simulation {
            state: state.clone(),
            dev_tools: DevTools::new(state),
            environment_stats: false,
            entity_stats,
        };

        sim
    }

    pub fn update(&mut self) {
        {
            for nom in self.state.borrow().noms.borrow().iter() {
                nom.borrow_mut()
                    .update(self.state.borrow().devtools.disable_movement);
            }
        }
        self.entity_stats.update(
            self.state.borrow().noms.clone(),
            &self.dev_tools.is_active(),
        );
        self.dev_tools.update();
        if is_key_pressed(KeyCode::I) {
            self.environment_stats = !self.environment_stats;
        }
        if is_key_pressed(KeyCode::Escape) {
            quit();
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.1020, 0.1804, 0.0196, 0.15),
        );
        self.state
            .borrow()
            .quadtree
            .borrow()
            .draw(self.state.borrow().devtools.visuals.quadtree);
        for nom in self.state.borrow().noms.borrow().iter() {
            nom.borrow().draw(self.state.clone());
        }
        self.dev_tools.draw();
        self.entity_stats.draw();

        if self.environment_stats || self.dev_tools.is_active() {
            self.draw_stats();
        }
    }

    pub fn draw_stats(&self) {
        let nom_amount = self.state.borrow().noms.borrow().len();
        let text = format!("Number of noms: {}", nom_amount);
        draw_text(&text, screen_width() - 200.0, 30.0, 20.0, WHITE);
    }
}
