use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

use crate::config::{
    BORDER_SPAWN_OFFSET, FOOD_SIZES, FOOD_SPAWN_SIZE_RATIO, MIN_FOOD_DISTANCE_APART, WINDOW_SIZE,
};
use crate::food::Food;
use crate::nom::Nom;

const BACKGROUND_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

pub struct Simulation {
    nom_arr: Vec<Nom>,
    food_arr: Vec<Food>,
    view_detection_visuals: bool,
}

impl Simulation {
    pub fn new(
        starting_noms_amount: u64,
        starting_food_amount: u64,
        view_detection_visuals: bool,
    ) -> Simulation {
        let mut sim = Simulation {
            nom_arr: Vec::with_capacity(starting_noms_amount as usize),
            food_arr: Vec::with_capacity(starting_food_amount as usize),
            view_detection_visuals,
        };
        sim.spawn_nom(starting_noms_amount);
        sim.spawn_food(starting_food_amount);
        return sim;
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        clear(BACKGROUND_COLOR, g);
        for food in &self.food_arr {
            food.draw(&c, g);
        }
        for nom in &self.nom_arr {
            nom.draw(&c, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        for nom in &mut self.nom_arr {
            nom.check_detection_range(&self.food_arr);
            nom.move_forward(delta_time);
        }
    }

    pub fn spawn_food(&mut self, starting_food_amount: u64) {
        let mut rng = thread_rng();
        let total_ratio: u64 = FOOD_SPAWN_SIZE_RATIO.iter().sum();
        let mut size_index = 0;
        let mut food_size = 0;

        for _ in 0..starting_food_amount {
            let mut valid_position = false;
            let mut gen_food_x = 0;
            let mut gen_food_y = 0;

            while !valid_position {
                gen_food_x = rng
                    .gen_range(BORDER_SPAWN_OFFSET..(WINDOW_SIZE[0] as u64) - BORDER_SPAWN_OFFSET);
                gen_food_y = rng
                    .gen_range(BORDER_SPAWN_OFFSET..(WINDOW_SIZE[1] as u64) - BORDER_SPAWN_OFFSET);

                let ratio_index = (size_index % total_ratio) as u64;
                let mut cumulative_ratio = 0;
                for (i, &ratio) in FOOD_SPAWN_SIZE_RATIO.iter().enumerate() {
                    cumulative_ratio += ratio;
                    if ratio_index < cumulative_ratio {
                        food_size = FOOD_SIZES[i];
                        break;
                    }
                }
                size_index += 1;
                // Check if this position
                valid_position = true;
                for food in &self.food_arr {
                    let dx = food.position[0] as f64 - gen_food_x as f64;
                    let dy = food.position[1] as f64 - gen_food_y as f64;
                    let distance = (dx * dx + dy * dy).sqrt();
                    if distance < MIN_FOOD_DISTANCE_APART {
                        valid_position = false;
                        break;
                    }
                }
            }
            self.food_arr
                .push(Food::new([gen_food_x, gen_food_y], food_size));
        }
    }

    pub fn spawn_nom(&mut self, starting_noms_amount: u64) {
        for _i in 0..starting_noms_amount {
            self.nom_arr.push(Nom::new(self.view_detection_visuals));
        }
    }
    // fn is_position_valid(&self, position: (u32, u32)) -> bool {
    //     for square in &self.food_arr {
    //         let dx = square.position.0 as i32 - position.0 as i32;
    //         let dy = square.position.1 as i32 - position.1 as i32;
    //         let distance_squared = dx * dx + dy * dy;

    //         // Ensure distance is greater than MIN_DISTANCE
    //         if distance_squared < (MIN_FOOD_DISTANCE_APART * MIN_FOOD_DISTANCE_APART) as i32 {
    //             return false;
    //         }
    //     }
    //     true
    // }
    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::F => {
                self.spawn_food(1);
            }
            _ => (),
        }
    }
}
