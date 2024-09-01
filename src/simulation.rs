use macroquad::prelude::*;
use miniquad::window::quit;

use crate::dev_tools::DevTools;
use crate::nom::Nom;
use crate::plants::{PlantSpawn, Plants};

const PROD_PLANTS: PlantSpawn = PlantSpawn {
    xs: 20,
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
    stats_active: bool,
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
                    stats_active: false,
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
                noms: Vec::new(),
                plants: Plants::new(PROD_PLANTS),
                testing_visuals,
                dev_tools: DevTools::new(),
                stats_active: false,
            },
            SimulationType::NomsOnly => Simulation {
                noms: Vec::new(),
                plants: Plants::empty(),
                testing_visuals,
                dev_tools: DevTools::new(),
                stats_active: false,
            },
            SimulationType::PlantOnly => Simulation {
                noms: Vec::new(),
                plants: Plants::new(PROD_PLANTS),
                testing_visuals,
                dev_tools: DevTools::new(),
                stats_active: false,
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
        if self.stats_active || self.dev_tools.devtools_active {
            self.draw_stats();
        }
    }

    pub fn update(&mut self) {
        for nom in &mut self.noms {
            // nom.check_detection_range(&self.plants.plant_vec);
            nom.move_forward();
        }
    }

    pub fn key_pressed(&mut self) {
        self.dev_tools.handle_inputs();
        if is_key_pressed(KeyCode::I) {
            self.stats_active = !self.stats_active;
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
            self.noms.push(Nom::new(self.testing_visuals));
        }
    }
}
