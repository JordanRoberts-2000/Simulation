use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::f32::consts::PI;

// use crate::plants::Plant;

const NOM_SPAWN_SIZE: f32 = 14.;
const NOM_SPAWN_SPEED: f32 = 40.;
const NOM_SPAWN_DETECTION_RADIUS: f32 = 100.;
const NOM_BORDER_COLOR: Color = Color::new(0.9961, 0.0, 0.9961, 1.0);
const NOM_COLOR: Color = Color::new(0.2, 0.0, 0.2, 1.0);

enum TargetType {
    Food,
    RandomPosition,
    Running,
}

pub struct Nom {
    pub size: f32,
    pub position: [f32; 2],
    pub speed: f32,
    target_position: Option<[f32; 2]>,
    target_type: Option<TargetType>,
    testing_visuals: bool,
}

impl Nom {
    pub fn new(testing_visuals: bool) -> Nom {
        let mut rng = thread_rng();
        let border_spawn_offset = 8;
        let position_x =
            rng.gen_range(border_spawn_offset..screen_width() as u32 - border_spawn_offset);
        let position_y =
            rng.gen_range(border_spawn_offset..screen_height() as u32 - border_spawn_offset);
        return Nom {
            size: NOM_SPAWN_SIZE,
            position: [position_x as f32, position_y as f32],
            speed: NOM_SPAWN_SPEED,
            target_position: None,
            target_type: None,
            testing_visuals,
        };
    }

    pub fn draw(&self) {
        draw_circle(
            self.position[0],
            self.position[1],
            self.size / 2.0,
            if self.size > 3.0 {
                NOM_BORDER_COLOR
            } else {
                NOM_COLOR
            },
        );
        if self.size > 3.0 {
            draw_circle(
                self.position[0],
                self.position[1],
                (self.size / 2.0) - 2.0,
                NOM_COLOR,
            );
        }
        if self.testing_visuals {
            draw_circle_lines(
                self.position[0],
                self.position[1],
                NOM_SPAWN_DETECTION_RADIUS,
                0.5,
                WHITE,
            );
        }
    }

    pub fn move_forward(&mut self) {
        let delta_time = get_frame_time();
        if let Some(target) = self.target_position {
            // Calculate the direction vector towards the target
            let direction_x = target[0] - self.position[0];
            let direction_y = target[1] - self.position[1];

            // Calculate the distance to the target
            let distance_to_target = (direction_x.powi(2) + direction_y.powi(2)).sqrt();

            // Calculate the distance the AI would travel in this update
            let travel_distance = self.speed * delta_time;

            // Check if the AI would overshoot the target
            if travel_distance >= distance_to_target {
                // Set the position to the target if it would overshoot
                self.position[0] = target[0];
                self.position[1] = target[1];
                self.target_position = None;
                return;
            }

            // Normalize the direction vector
            let direction_x_normalized = direction_x / distance_to_target;
            let direction_y_normalized = direction_y / distance_to_target;

            // Move the AI towards the target
            self.position[0] += direction_x_normalized * travel_distance;
            self.position[1] += direction_y_normalized * travel_distance;
        }
    }
    // pub fn check_detection_range(&mut self, food_arr: &Vec<Plant>) {
    //     // is threat near me?

    //     // is food near me?
    //     // toDo: improve logic so all food in range is identified, then closest is chosen then return prematurely,
    //     //  also check if food is now being targeted by a closer nom
    //     // for food in food_arr {
    //     //     let dx = food.position[0] as f64 - self.position[0] as f64;
    //     //     let dy = food.position[1] as f64 - self.position[1] as f64;
    //     //     let distance = (dx * dx + dy * dy).sqrt();
    //     //     if distance <= NOM_SPAWN_DETECTION_RADIUS as f64 / 2.0 {
    //     //         self.target_position = Some(food.position);
    //     //     }
    //     // }

    //     // set to random position
    //     if self.target_position.is_none() {
    //         let mut rng = thread_rng();
    //         let radius: f32 = 50.0;
    //         let margin: f32 = 10.0;

    //         loop {
    //             // Generate a random angle between 0 and 2π radians
    //             let angle: f32 = rng.gen_range(0.0..2.0 * PI);

    //             // Calculate the x and y coordinates on the circumference
    //             let target_x = self.position[0] + radius * angle.cos();
    //             let target_y = self.position[1] + radius * angle.sin();

    //             // Check if the target position is within 10px of the screen borders
    //             if target_x > margin
    //                 && target_x < (screen_width() - margin)
    //                 && target_y > margin
    //                 && target_y < (screen_height() - margin)
    //             {
    //                 // Valid position, set as target_position
    //                 self.target_position = Some([target_x, target_y]);
    //                 break;
    //             }
    //         }
    //     }
    // }
}
