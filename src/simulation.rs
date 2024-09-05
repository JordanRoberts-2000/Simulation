use macroquad::prelude::*;
use miniquad::window::quit;

use crate::dev_tools::DevTools;
use crate::entity_stats::EntityStats;
use crate::nom::Nom;
use crate::plants::{PlantSpawn, Plants};

const PROD_PLANTS: PlantSpawn = PlantSpawn {
    xs: 1,
    sm: 20,
    md: 2,
    lg: 1,
    xl: true,
};

pub enum SimulationType {
    Production,
    Custom,
    PlantOnly,
    NomsOnly,
}

pub struct Simulation {
    noms: Vec<Nom>,
    plants: Plants,
    testing_visuals: bool,
    dev_tools: DevTools,
    environment_stats: bool,
    entity_stats: EntityStats,
}

impl Simulation {
    pub fn new(sim_type: SimulationType, testing_visuals: bool) -> Simulation {
        let mut sim = match sim_type {
            SimulationType::Custom => {
                let mut custom_sim = Simulation {
                    noms: Vec::new(),
                    plants: Plants::empty(),
                    testing_visuals,
                    dev_tools: DevTools::new(),
                    environment_stats: false,
                    entity_stats: EntityStats::new(),
                };
                // custom_sim.spawn_plant(size, position);
                // custom_sim.spawn_plants(PlantSpawn {
                //     xs: 20,
                //     sm: 20,
                //     md: 2,
                //     lg: 1,
                //     xl: true,
                // });
                return custom_sim;
            }
            SimulationType::Production => Simulation {
                noms: vec![
                    Nom::new(vec2(200., 200.), true),
                    Nom::new(vec2(500., 200.), false),
                    Nom::new(vec2(500., 300.), false),
                    Nom::new(vec2(500., 400.), false),
                    Nom::new(vec2(500., 500.), false),
                    Nom::new(vec2(500., 600.), false),
                    Nom::new(vec2(500., 700.), false),
                    Nom::new(vec2(700., 500.), false),
                    Nom::new(vec2(700., 600.), false),
                    Nom::new(vec2(700., 400.), false),
                    Nom::new(vec2(700., 300.), false),
                    Nom::new(vec2(700., 200.), false),
                    Nom::new(vec2(700., 100.), false),
                ],
                plants: Plants::new(PROD_PLANTS),
                testing_visuals,
                dev_tools: DevTools::new(),
                environment_stats: false,
                entity_stats: EntityStats::new(),
            },
            SimulationType::NomsOnly => Simulation {
                noms: Vec::new(),
                plants: Plants::empty(),
                testing_visuals,
                dev_tools: DevTools::new(),
                environment_stats: false,
                entity_stats: EntityStats::new(),
            },
            SimulationType::PlantOnly => Simulation {
                noms: Vec::new(),
                plants: Plants::new(PROD_PLANTS),
                testing_visuals,
                dev_tools: DevTools::new(),
                environment_stats: false,
                entity_stats: EntityStats::new(),
            },
            _ => panic!("Invalid simulation type"),
        };
        return sim;
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

        for plant in &self.plants.plant_vec {
            plant.draw();
        }
        for nom in &self.noms {
            nom.draw();
        }
        self.dev_tools.draw();
        self.entity_stats.draw(&self.noms);

        if self.environment_stats || self.dev_tools.devtools_active {
            self.draw_stats();
        }
    }

    pub fn update(&mut self) {
        let noms_len = self.noms.len();
        for nom in &mut self.noms {
            nom.update();
        }
        // check in pairs, avoid duplicate checks: a -> b & b -> a;
        for i in 0..noms_len {
            let mut is_colliding = false; // Track whether nom_a has collided with any other nom
            for j in (i + 1)..noms_len {
                let (left, right) = self.noms.split_at_mut(j);
                let nom_a = &mut left[i];
                let nom_b = &mut right[0];

                if nom_a.check_collision(nom_b) {
                    // If a collision is detected, mark it
                    is_colliding = true;
                    nom_b.temp_is_colliding = true; // Also mark nom_b as colliding
                }
            }
            // After checking all other noms, update nom_a's collision status
            self.noms[i].temp_is_colliding = is_colliding;
        }
        // collision detection:
        // 1.visibly show collision
        // 1. check *mut is nessesary
        // 2. spacial hashmaps
        // 3. collision w/wall
        // ------------------------
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

    pub fn spawn_nom(&mut self, starting_noms_amount: u32) {
        for _i in 0..starting_noms_amount {
            // self.noms.push(Nom::new());
        }
    }
}
