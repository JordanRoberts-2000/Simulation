use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

pub mod cluster;
pub mod plant;

use cluster::{Cluster, ClusterSize};
use plant::{Plant, PlantSize};

const PLANT_COLOR: Color = Color::new(0.0, 0.97, 0.0, 1.0);
pub const MIN_PLANT_DISTANCE_APART: f32 = 30.0;
pub const PLANT_SPAWN_SIZE_RATIO: [u32; 3] = [8, 2, 1]; // small -> big
pub const PLANT_SIZES: [f32; 3] = [1.0, 6.0, 8.0];
const BORDER_SPAWN_OFFSET: u32 = 20;

pub struct PlantSpawn {
    pub xs: u32,
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: bool,
}

pub struct Plants {
    pub plant_vec: Vec<Plant>,
    cluster_vec: Vec<[u32; 2]>,
}

impl Plants {
    pub fn new(spawn_plants: PlantSpawn) -> Plants {
        let mut plant_vec: Vec<Plant> = Vec::new();
        let mut cluster_vec: Vec<[u32; 2]> = Vec::new();
        if spawn_plants.xl {
            // Cluster::spawn(1, ClusterSize::xl, &mut plant_vec, &mut plant_vec);
        }
        // Cluster::spawn(spawn_plants.lg, ClusterSize::lg, &mut plant_vec, &mut cluster_vec);
        // Cluster::spawn(spawn_plants.md, ClusterSize::md, &mut plant_vec, &mut cluster_vec);
        // Cluster::spawn(spawn_plants.sm, ClusterSize::sm, &mut plant_vec, &mut cluster_vec);
        // Cluster::spawn(spawn_plants.xs, ClusterSize::xs, &mut plant_vec, &mut cluster_vec);
        return Plants {
            plant_vec,
            cluster_vec,
        };
    }
}

// =====================================================================================
// pub fn new(spawn_plants: PlantSpawn) -> Plants {
//   // spawn center large cluster
//   // spawn blue cluster
//   // spawn large clusters in preferably on screen edges
//   // spawn medium clusters where there is space from previous cluster positions
//   // spawn sm & xs clusters where there is space from smaller plants
//   // IMPORTANT: random squares should work by splitting the grid in quadrants to spread them out more
//   let mut plant_vec: Vec<Plant> = Vec::new();
//   if spawn_plants.xl {
//       // let cluster_xl = Cluster::new(ClusterSize::xl);
//       // cluster_vec.push(cluster_xl.position);
//       // for plant in 0..cluster_xl.plant_vec {
//       //    plant_vec.push(plant);
//       // }
//   }
//   // spawn big ones first
//   for _ in 0..spawn_plants.xs {
//       // check positions away from current positions
//       // let cluster = Cluster::new(ClusterSize::Xs, [300., 300.]);
//       // plant_vec.push();
//   }
//   return Plants {
//       plant_vec,
//       cluster_vec: Vec::new(),
//   };
// }
// =====================================================================================

// pub fn get_position(existing_plants: &Vec<Plant>) -> [f32; 2] {
//     let mut rng = thread_rng();
//     let mut valid_position = false;
//     let mut gen_plant_x = 0;
//     let mut gen_plant_y = 0;

//     while !valid_position {
//         gen_plant_x =
//             rng.gen_range(BORDER_SPAWN_OFFSET..screen_width() as u32 - BORDER_SPAWN_OFFSET);
//         gen_plant_y =
//             rng.gen_range(BORDER_SPAWN_OFFSET..screen_height() as u32 - BORDER_SPAWN_OFFSET);
//         valid_position = true;
//         for plant in existing_plants {
//             let dx = plant.position[0] - gen_plant_x as f32;
//             let dy = plant.position[1] - gen_plant_y as f32;
//             let distance = (dx * dx + dy * dy).sqrt();
//             if distance < MIN_PLANT_DISTANCE_APART {
//                 valid_position = false;
//                 break;
//             }
//         }
//     }
//     return [gen_plant_x as f32, gen_plant_y as f32];
// }

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
