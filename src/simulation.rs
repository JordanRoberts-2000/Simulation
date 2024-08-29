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
    waiting_time: f64,
}

impl Simulation {
    pub fn new(starting_noms_amount: u64, starting_food_amount: u64) -> Simulation {
        let nom_arr = Simulation::spawn_noms(starting_noms_amount);
        let food_arr = Simulation::spawn_food(starting_food_amount);
        return Simulation {
            nom_arr,
            food_arr,
            waiting_time: 0.0,
        };
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        clear(BACKGROUND_COLOR, g);
        for food in &self.food_arr {
            Food::draw(food.position, food.size, &c, g);
        }
        for nom in &self.nom_arr {
            Nom::draw(nom.position, nom.size, &c, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
    }

    fn spawn_food(starting_food_amount: u64) -> Vec<Food> {
        let mut rng = thread_rng();
        let mut food_vec: Vec<Food> = Vec::with_capacity(starting_food_amount as usize);
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
                for food in &food_vec {
                    let dx = food.position[0] as f64 - gen_food_x as f64;
                    let dy = food.position[1] as f64 - gen_food_y as f64;
                    let distance = (dx * dx + dy * dy).sqrt();
                    if distance < MIN_FOOD_DISTANCE_APART {
                        valid_position = false;
                        break;
                    }
                }
            }
            food_vec.push(Food::new([gen_food_x, gen_food_y], food_size));
        }

        food_vec
    }

    fn spawn_noms(starting_noms_amount: u64) -> Vec<Nom> {
        let mut rng = thread_rng();
        let mut nom_vec: Vec<Nom> = Vec::new();
        for _i in 0..starting_noms_amount {
            nom_vec.push(Nom::new());
        }
        return nom_vec;
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
    // pub fn key_pressed(&mut self, key: Key) {
    //     let dir = match key {
    //         Key::Up => Some(Direction::Up),
    //         Key::Down => Some(Direction::Down),
    //         Key::Left => Some(Direction::Left),
    //         Key::Right => Some(Direction::Right),
    //         _ => Some(self.snake.head_direction()),
    //     };
    //     if let Some(dir) = dir {
    //         if dir == self.snake.head_direction().opposite() {
    //             return;
    //         }
    //     }
    // }
}
