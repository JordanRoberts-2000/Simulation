use macroquad::prelude::*;
use miniquad::window::quit;

use crate::dev_tools::DevTools;
use crate::entity_stats::EntityStats;
use crate::nom::Nom;
use crate::plants::{PlantSpawn, Plants};

const DEFAULT_PLANTS_SPAWN: PlantSpawn = PlantSpawn {
    xs: 1,
    sm: 20,
    md: 2,
    lg: 1,
    xl: true,
};

pub struct Simulation {
    noms: Vec<Nom>,
    plants: Plants,
    dev_tools: DevTools,
    environment_stats: bool,
    entity_stats: EntityStats,
}

impl Simulation {
    pub fn new() -> Simulation {
        return Simulation {
            noms: vec![
                Nom::new(vec2(200., 200.), true),
                Nom::new(vec2(500., 200.), false),
                Nom::new(vec2(500., 300.), false),
                Nom::new(vec2(500., 400.), false),
                Nom::new(vec2(500., 500.), false),
            ],
            plants: Plants::new(DEFAULT_PLANTS_SPAWN),
            dev_tools: DevTools::new(),
            environment_stats: false,
            entity_stats: EntityStats::new(),
        };
    }

    pub fn update(&mut self) {
        for nom in &mut self.noms {
            nom.update();
        }
        self.check_collions();
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

    pub fn check_collions(&mut self) {
        // check in pairs, avoid duplicate checks: a -> b & b -> a;
        let noms_len = self.noms.len();
        for i in 0..noms_len {
            let mut is_colliding_a = false;
            for j in (i + 1)..noms_len {
                let (left, right) = self.noms.split_at_mut(j);
                let nom_a = &mut left[i];
                let nom_b = &mut right[0];

                if nom_a.check_collision(nom_b) {
                    is_colliding_a = true;
                    self.noms[j].temp_is_colliding = true;
                }
                // match nom_a.check_collision(nom_b) {
                //     ColissionEnum::Nom => nom_a.collided(nom_b),
                //     ColissionEnum::Wall => nom_a.collided_into_wall(),
                // }
            }
            self.noms[i].temp_is_colliding = is_colliding_a;
        }
    }
}
