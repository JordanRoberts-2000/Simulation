use macroquad::prelude::*;

use super::{utils::lerp_angle, Nom};

impl Nom {
    pub fn update(&mut self) {
        let delta_time = get_frame_time();
        self.update_orientation(delta_time);
        self.update_position(&delta_time);
    }
    fn update_position(&mut self, delta_time: &f32) {
        self.current_speed =
            (self.current_speed + self.acceleration * delta_time).min(self.max_speed);
        self.position.x += self.current_speed * self.orientation.to_radians().cos() * delta_time;
        self.position.y += self.current_speed * self.orientation.to_radians().sin() * delta_time;
        self.clamp_position_to_screen();
    }

    fn update_orientation(&mut self, delta_time: f32) {
        let wandering_force = self.calculate_wandering_behavior();
        let avoidance_force = self.calculate_avoidance_behavior();

        // Blend the behaviors with weights
        let final_force = (wandering_force * 0.6) + (avoidance_force * 0.4);

        // Use the blended force to calculate the target orientation
        self.target_orientation = wandering_force;
        self.orientation = lerp_angle(
            self.orientation,
            self.target_orientation,
            self.turning_speed,
            delta_time,
        );
    }
}
