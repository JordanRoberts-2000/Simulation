use macroquad::prelude::*;
use miniquad::window::quit;

use crate::nom::Nom;
use crate::plant::Plant;

const BACKGROUND_COLOR: Color = BLACK;

pub struct Simulation {
    nom_arr: Vec<Nom>,
    plant_arr: Vec<Plant>,
    view_detection_visuals: bool,
}

impl Simulation {
    pub fn new(
        starting_noms_amount: u32,
        starting_plant_amount: u32,
        view_detection_visuals: bool,
    ) -> Simulation {
        let mut sim = Simulation {
            nom_arr: Vec::with_capacity(starting_noms_amount as usize),
            plant_arr: Vec::with_capacity(starting_plant_amount as usize),
            view_detection_visuals,
        };
        sim.spawn_nom(starting_noms_amount);
        sim.spawn_plant(starting_plant_amount);
        return sim;
    }

    pub fn draw(&self) {
        clear_background(BACKGROUND_COLOR);
        for plant in &self.plant_arr {
            plant.draw();
        }
        for nom in &self.nom_arr {
            nom.draw();
        }
    }

    pub fn update(&mut self) {
        for nom in &mut self.nom_arr {
            nom.check_detection_range(&self.plant_arr);
            nom.move_forward();
        }
    }

    pub fn spawn_plant(&mut self, plant_amount: u32) {
        for _ in 0..plant_amount {
            self.plant_arr.push(Plant::new(&self.plant_arr));
        }
    }

    pub fn spawn_nom(&mut self, starting_noms_amount: u32) {
        for _i in 0..starting_noms_amount {
            self.nom_arr.push(Nom::new(self.view_detection_visuals));
        }
    }

    pub fn key_pressed(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            quit();
        }
        if is_key_pressed(KeyCode::F) {
            self.spawn_plant(1);
        }
    }
}
