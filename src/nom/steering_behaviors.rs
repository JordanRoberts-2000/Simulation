use macroquad::prelude::*;
use noise::NoiseFn;

use super::{utils::map_range, Nom};

impl Nom {
    pub fn calculate_avoidance_behavior(&mut self) -> f32 {
        let radius = self.size / 2.0;
        let margin = 40.0; // Distance from the wall at which to start steering
        let orientation = self.orientation; // The current orientation of the circle

        // Check proximity to the left wall
        if self.position.x - radius <= margin {
            let target_angle = 0.0; // Wall is to the left, we need to steer right
            return self.steer_away(orientation, target_angle);
        }
        // Check proximity to the right wall
        else if self.position.x + radius >= screen_width() - margin {
            let target_angle = std::f32::consts::PI; // Wall is to the right, steer left
            return self.steer_away(orientation, target_angle);
        }

        // Check proximity to the top wall
        if self.position.y - radius <= margin {
            let target_angle = std::f32::consts::FRAC_PI_2; // Wall is above, steer down
            return self.steer_away(orientation, target_angle);
        }
        // Check proximity to the bottom wall
        else if self.position.y + radius >= screen_height() - margin {
            let target_angle = 3.0 * std::f32::consts::FRAC_PI_2; // Wall is below, steer up
            return self.steer_away(orientation, target_angle);
        }

        // No need to steer
        0.0
    }

    fn steer_away(&self, current_orientation: f32, target_angle: f32) -> f32 {
        // Calculate the difference between the current orientation and the target
        let mut angle_diff = target_angle - current_orientation;

        // Normalize the angle difference to be between -π and π
        if angle_diff > std::f32::consts::PI {
            angle_diff -= 2.0 * std::f32::consts::PI;
        } else if angle_diff < -std::f32::consts::PI {
            angle_diff += 2.0 * std::f32::consts::PI;
        }

        // Determine the direction to steer
        if angle_diff > 0.0 {
            // Turn right
            std::f32::consts::FRAC_PI_2 // Steer right (positive direction)
        } else {
            // Turn left
            -std::f32::consts::FRAC_PI_2 // Steer left (negative direction)
        }
    }

    pub fn calculate_wandering_behavior(&mut self) -> f32 {
        // Step 1: Calculate the center of the look-ahead circle in front of the agent
        let look_ahead_center = Vec2::new(
            self.position.x + self.orientation.cos() * self.look_ahead_distance,
            self.position.y + self.orientation.sin() * self.look_ahead_distance,
        );

        // Step 2: Use Perlin noise to determine the angle around the look-ahead circle
        let noise_frequency = 0.6; // Adjust the frequency for smoother changes
        let noise_value = self
            .perlin
            .get([self.time_offset + get_time() * noise_frequency]);

        // Step 3: Map the noise value (-1.0 to 1.0) to a smaller range around the orientation
        // Let's map it to a small arc range for more control (like -45 to +45 degrees)
        let angle_deviation = map_range(
            noise_value,
            -1.0,
            1.0,
            -std::f32::consts::FRAC_PI_3,
            std::f32::consts::FRAC_PI_3,
        );

        // Step 4: Add the agent's current orientation to bias the wandering target forward
        let angle_on_circle =
            (self.orientation + angle_deviation).rem_euclid(2.0 * std::f32::consts::PI);

        // Step 5: Calculate the target point on the circumference of the look-ahead circle
        self.look_ahead_target = Vec2::new(
            look_ahead_center.x + self.look_ahead_size / 2.0 * angle_on_circle.cos(),
            look_ahead_center.y + self.look_ahead_size / 2.0 * angle_on_circle.sin(),
        );

        // Step 6: Calculate the angle to the target point in radians
        let angle_to_target = (self.look_ahead_target.y - self.position.y)
            .atan2(self.look_ahead_target.x - self.position.x)
            .rem_euclid(2.0 * std::f32::consts::PI);

        angle_to_target // Return the target angle in radians
    }
}
