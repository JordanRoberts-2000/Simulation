use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

const PLANT_COLOR: Color = Color::new(0.0, 0.97, 0.0, 1.0);
pub const MIN_PLANT_DISTANCE_APART: f32 = 30.0;
pub const PLANT_SPAWN_SIZE_RATIO: [u32; 3] = [8, 2, 1]; // small -> big
pub const PLANT_SIZES: [f32; 3] = [1.0, 6.0, 8.0];
const BORDER_SPAWN_OFFSET: u32 = 20;

pub enum PlantSize {
    Small,
    Medium,
    Large,
}

impl PlantSize {
    fn size_value(&self) -> f32 {
        match self {
            PlantSize::Small => PLANT_SIZES[0],
            PlantSize::Medium => PLANT_SIZES[1],
            PlantSize::Large => PLANT_SIZES[2],
        }
    }
}

pub struct Plant {
    position: [f32; 2],
    size: PlantSize,
}

impl Plant {
    pub fn new(existing_plants: &Vec<Plant>) -> Plant {
        let position = Plant::get_position(&existing_plants);
        let size = Plant::get_size();
        return Plant { position, size };
    }

    pub fn draw(&self) {
        for i in 0..10 {
            let size = 0.75;
            let radius = size + i as f32 * size;
            let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
            let color = Color::new(0.0, 0.9, 0.0, alpha); // Semi-transparent color

            draw_circle(self.position[0], self.position[1], radius, color);
        }
    }

    pub fn get_position(existing_plants: &Vec<Plant>) -> [f32; 2] {
        let mut rng = thread_rng();
        let mut valid_position = false;
        let mut gen_plant_x = 0;
        let mut gen_plant_y = 0;

        while !valid_position {
            gen_plant_x =
                rng.gen_range(BORDER_SPAWN_OFFSET..screen_width() as u32 - BORDER_SPAWN_OFFSET);
            gen_plant_y =
                rng.gen_range(BORDER_SPAWN_OFFSET..screen_height() as u32 - BORDER_SPAWN_OFFSET);
            valid_position = true;
            for plant in existing_plants {
                let dx = plant.position[0] - gen_plant_x as f32;
                let dy = plant.position[1] - gen_plant_y as f32;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < MIN_PLANT_DISTANCE_APART {
                    valid_position = false;
                    break;
                }
            }
        }
        return [gen_plant_x as f32, gen_plant_y as f32];
    }

    pub fn get_size() -> PlantSize {
        return PlantSize::Small;
    }
}

// let mut rng = thread_rng();
//         let total_ratio: u64 = PLANT_SPAWN_SIZE_RATIO.iter().sum();
//         let mut size_index = 0;
//         let mut food_size = 0;

//         for _ in 0..starting_plant_amount {
//             let mut valid_position = false;
//             let mut gen_food_x = 0;
//             let mut gen_food_y = 0;

//             while !valid_position {
//                 gen_food_x = rng
//                     .gen_range(BORDER_SPAWN_OFFSET..(WINDOW_SIZE[0] as u64) - BORDER_SPAWN_OFFSET);
//                 gen_food_y = rng
//                     .gen_range(BORDER_SPAWN_OFFSET..(WINDOW_SIZE[1] as u64) - BORDER_SPAWN_OFFSET);

//                 let ratio_index = (size_index % total_ratio) as u64;
//                 let mut cumulative_ratio = 0;
//                 for (i, &ratio) in FOOD_SPAWN_SIZE_RATIO.iter().enumerate() {
//                     cumulative_ratio += ratio;
//                     if ratio_index < cumulative_ratio {
//                         food_size = FOOD_SIZES[i];
//                         break;
//                     }
//                 }
//                 size_index += 1;
//                 // Check if this position
//                 valid_position = true;
//                 for food in &self.food_arr {
//                     let dx = food.position[0] as f64 - gen_food_x as f64;
//                     let dy = food.position[1] as f64 - gen_food_y as f64;
//                     let distance = (dx * dx + dy * dy).sqrt();
//                     if distance < MIN_FOOD_DISTANCE_APART {
//                         valid_position = false;
//                         break;
//                     }
//                 }
//             }
