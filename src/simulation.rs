use std::cell::RefCell;
use std::rc::Rc;

use macroquad::prelude::*;
use miniquad::window::quit;

use crate::dev_tools::DevTools;
use crate::entity_stats::EntityStats;
use crate::nom::{Nom, Variant};
use crate::plants::{PlantSpawn, Plants};
use crate::quadtree::Quadtree;

const DEFAULT_PLANTS_SPAWN: PlantSpawn = PlantSpawn {
    xs: 1,
    sm: 20,
    md: 2,
    lg: 1,
    xl: true,
};

pub struct Simulation {
    noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
    plants: Plants,
    dev_tools: DevTools,
    environment_stats: bool,
    entity_stats: EntityStats,
    quadtree: Rc<RefCell<Quadtree>>,
}

impl Simulation {
    pub fn new() -> Simulation {
        let noms = Rc::new(RefCell::new(vec![
            Rc::new(RefCell::new(Nom::new(vec2(300., 100.), Variant::Default))),
            Rc::new(RefCell::new(Nom::new(
                vec2(300., 300.),
                Variant::DefaultTwin,
            ))),
            Rc::new(RefCell::new(Nom::new(
                vec2(500., 100.),
                Variant::WhaleMutation,
            ))),
            Rc::new(RefCell::new(Nom::new(vec2(500., 300.), Variant::Whale))),
            Rc::new(RefCell::new(Nom::new(vec2(700., 100.), Variant::Hedgehog))),
            Rc::new(RefCell::new(Nom::new(
                vec2(700., 300.),
                Variant::HedgehogMutation,
            ))),
            Rc::new(RefCell::new(Nom::new(vec2(900., 100.), Variant::Wendigo))),
            Rc::new(RefCell::new(Nom::new(vec2(900., 300.), Variant::Wendigo2))),
            Rc::new(RefCell::new(Nom::new(vec2(1100., 100.), Variant::Death))),
            Rc::new(RefCell::new(Nom::new(vec2(1100., 300.), Variant::Zombie))),
            Rc::new(RefCell::new(Nom::new(vec2(700., 700.), Variant::Leviathan))),
            // ... other noms
        ]));
        let quadtree = Rc::new(RefCell::new(Quadtree::new()));
        let sim = Simulation {
            noms: noms.clone(), // Use the same noms in Simulation
            plants: Plants::new(DEFAULT_PLANTS_SPAWN),
            dev_tools: DevTools::new(noms.clone(), quadtree.clone()),
            environment_stats: false,
            entity_stats: EntityStats::new(),
            quadtree: quadtree.clone(),
        };

        // Insert noms into the quadtree
        for nom in noms.borrow().iter() {
            sim.quadtree.borrow_mut().insert(Rc::clone(nom));
        }

        sim
    }

    pub fn update(&mut self) {
        {
            let noms = self.noms.borrow(); // Borrow noms immutably
            for nom in noms.iter() {
                nom.borrow_mut().update();
            }
        }
        self.key_pressed();
    }

    pub fn key_pressed(&mut self) {
        self.dev_tools.handle_inputs();
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
        self.quadtree
            .borrow()
            .draw(self.dev_tools.quadtree_visuals_active.clone());
        for plant in &self.plants.plant_vec {
            plant.draw();
        }
        for nom in self.noms.borrow().iter() {
            nom.borrow().draw(self.dev_tools.nom_visuals_active.clone());
        }
        self.dev_tools.draw();
        // self.entity_stats.draw(&self.noms);

        if self.environment_stats || self.dev_tools.is_active() {
            self.draw_stats();
        }
    }

    pub fn draw_stats(&self) {
        draw_text("Number of noms:", screen_width() - 200.0, 30.0, 20.0, WHITE);
        draw_text(
            "Number of Plants:",
            screen_width() - 200.0,
            60.0,
            20.0,
            WHITE,
        );
    }
}
