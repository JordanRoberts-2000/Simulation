use piston_window::types::Color;
use piston_window::{Context, G2d};
use rand::{thread_rng, Rng};
use std::f64::consts::PI;

use crate::config::WINDOW_SIZE;
use crate::draw::{draw_circle, draw_hollow_circle, draw_line};
use crate::food::Food;

const NOM_SPAWN_SIZE: u64 = 6;
const NOM_SPAWN_SPEED: f64 = 40.0;
const NOM_SPAWN_DETECTION_RADIUS: u64 = 100;
const NOM_COLOR: Color = [0.7, 0.0, 0.7, 1.0];
const NOM_BORDER_COLOR: Color = [0.4, 0.0, 0.4, 1.0];

enum TargetType {
    Food,
    RandomPosition,
    Running,
}

pub struct Nom {
    pub size: u64,
    pub position: [f64; 2],
    pub speed: f64,
    target_position: Option<[u64; 2]>,
    target_type: Option<TargetType>,
    view_detection_visuals: bool,
}

impl Nom {
    pub fn new(view_detection_visuals: bool) -> Nom {
        let mut rng = thread_rng();
        let border_spawn_offset = 8;
        let position_x =
            rng.gen_range(border_spawn_offset..(WINDOW_SIZE[0] as u64) - border_spawn_offset);
        let position_y =
            rng.gen_range(border_spawn_offset..(WINDOW_SIZE[1] as u64) - border_spawn_offset);
        return Nom {
            size: NOM_SPAWN_SIZE,
            position: [position_x as f64, position_y as f64],
            speed: NOM_SPAWN_SPEED,
            target_position: None,
            target_type: None,
            view_detection_visuals,
        };
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        let (x, y) = (self.position[0] as u64, self.position[1] as u64);
        draw_circle(
            if self.size > 3 {
                NOM_COLOR
            } else {
                NOM_BORDER_COLOR
            },
            [x, y],
            self.size,
            &c,
            g,
        );
        if self.size > 3 {
            draw_hollow_circle(NOM_BORDER_COLOR, [x, y], self.size, 1.0, 100, &c, g);
        }
        if self.view_detection_visuals {
            draw_hollow_circle(
                [1.0, 1.0, 1.0, 1.0],
                [x, y],
                NOM_SPAWN_DETECTION_RADIUS,
                0.5,
                100,
                c,
                g,
            );
        }
    }

    pub fn move_forward(&mut self, delta_time: f64) {
        if let Some(target) = self.target_position {
            if let Some(target) = self.target_position {
                // Calculate the direction vector towards the target
                let direction_x = target[0] as f64 - self.position[0];
                let direction_y = target[1] as f64 - self.position[1];

                // Calculate the distance to the target
                let distance_to_target = (direction_x.powi(2) + direction_y.powi(2)).sqrt();

                // Calculate the distance the AI would travel in this update
                let travel_distance = self.speed * delta_time;

                // Check if the AI would overshoot the target
                if travel_distance >= distance_to_target {
                    // Set the position to the target if it would overshoot
                    self.position[0] = target[0] as f64;
                    self.position[1] = target[1] as f64;
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
    }
    pub fn check_detection_range(&mut self, food_arr: &Vec<Food>) {
        // is threat near me?

        // is food near me?
        // toDo: improve logic so all food in range is identified, then closest is chosen then return prematurely,
        //  also check if food is now being targeted by a closer nom
        // for food in food_arr {
        //     let dx = food.position[0] as f64 - self.position[0] as f64;
        //     let dy = food.position[1] as f64 - self.position[1] as f64;
        //     let distance = (dx * dx + dy * dy).sqrt();
        //     if distance <= NOM_SPAWN_DETECTION_RADIUS as f64 / 2.0 {
        //         self.target_position = Some(food.position);
        //     }
        // }

        // set to random position
        if self.target_position.is_none() {
            let mut rng = rand::thread_rng();
            let radius = 100.0;
            let margin = 10.0;

            loop {
                // Generate a random angle between 0 and 2π radians
                let angle = rng.gen_range(0.0..2.0 * PI);

                // Calculate the x and y coordinates on the circumference
                let target_x = self.position[0] + radius * angle.cos();
                let target_y = self.position[1] + radius * angle.sin();

                // Check if the target position is within 10px of the screen borders
                if target_x > margin
                    && target_x < (WINDOW_SIZE[0] as f64 - margin)
                    && target_y > margin
                    && target_y < (WINDOW_SIZE[1] as f64 - margin)
                {
                    // Valid position, set as target_position
                    self.target_position = Some([target_x as u64, target_y as u64]);
                    break;
                }
            }
        }
    }
}
