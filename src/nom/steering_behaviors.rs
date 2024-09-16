use macroquad::prelude::*;
use noise::NoiseFn;

use super::{utils::map_range, Nom};

impl Nom {
    pub fn calculate_avoidance_behavior(&mut self) -> f32 {
        let radius = self.size / 2.0;

        // Check for proximity to screen edges
        if self.position.x - radius <= 20.0 {
            return 0.0; // Turn towards the right (0 radians)
        } else if self.position.x + radius >= screen_width() - 20.0 {
            return std::f32::consts::PI; // Turn towards the left (π radians)
        }

        if self.position.y - radius <= 20.0 {
            return std::f32::consts::FRAC_PI_2; // Turn downwards (π/2 radians)
        } else if self.position.y + radius >= screen_height() - 20.0 {
            return 3.0 * std::f32::consts::FRAC_PI_2; // Turn upwards (3π/2 radians)
        }

        // No avoidance needed, return the current orientation (already in radians)
        self.orientation
    }

    pub fn calculate_wandering_behavior(&mut self) -> f32 {
        // Step 1: Calculate the center of the look-ahead circle in front of the agent
        let look_ahead_center = Vec2::new(
            self.position.x + self.orientation.cos() * self.look_ahead_distance,
            self.position.y + self.orientation.sin() * self.look_ahead_distance,
        );

        // Step 2: Use Perlin noise to determine the angle around the look-ahead circle
        let noise_frequency = 0.5; // Adjust the frequency for smoother changes
        let noise_value = self.perlin.get([
            self.time_offset + get_time() * noise_frequency,
            (self.position.x as f64 + self.time_offset * 10.0) * 0.1,
            (self.position.y as f64 + self.time_offset * 10.0) * 0.1,
        ]);

        // Step 3: Map the noise value (-1.0 to 1.0) to a smaller range around the orientation
        // Let's map it to a small arc range for more control (like -45 to +45 degrees)
        let angle_deviation = map_range(
            noise_value,
            -1.0,
            1.0,
            -std::f32::consts::PI,
            std::f32::consts::PI,
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
