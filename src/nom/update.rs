use macroquad::prelude::*;
use std::f32::consts::PI;

use super::{utils::lerp_angle, Nom};

impl Nom {
    pub fn update(&mut self, can_move: bool) {
        let delta_time = get_frame_time();
        if can_move {
            self.update_orientation(delta_time);
            self.update_position(&delta_time);
        }
    }
    fn update_position(&mut self, delta_time: &f32) {
        self.current_speed =
            (self.current_speed + self.acceleration * delta_time).min(self.max_speed);
        self.position.x += self.current_speed * self.orientation.cos() * delta_time;
        self.position.y += self.current_speed * self.orientation.sin() * delta_time;
        self.clamp_position_to_screen();
    }

    fn update_orientation(&mut self, delta_time: f32) {
        let wandering_force = self.calculate_wandering_behavior();
        let avoidance_force = self.calculate_avoidance_behavior();

        // Blend the behaviors with weights
        let final_force = wandering_force + avoidance_force;

        // Use the blended force to calculate the target orientation
        self.target_orientation = final_force;
        // self.orientation += self.turning_speed * delta_time;
        self.orientation = lerp_angle(
            self.orientation,
            self.target_orientation,
            self.turning_speed,
            delta_time,
        );
    }
}
