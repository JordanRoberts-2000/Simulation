use macroquad::prelude::*;
use noise::NoiseFn;

use super::{utils::map_range, Nom};

impl Nom {
    pub fn calculate_avoidance_behavior(&mut self) -> f32 {
        let radius = self.size / 2.0;

        // Check for proximity to screen edges
        if self.position.x - radius <= 20.0 {
            return 0.0; // Turn towards the right
        } else if self.position.x + radius >= screen_width() - 20.0 {
            return 180.0; // Turn towards the left
        }

        if self.position.y - radius <= 20.0 {
            return 90.0; // Turn downwards
        } else if self.position.y + radius >= screen_height() - 20.0 {
            return 270.0; // Turn upwards
        }

        // No avoidance needed, return the current orientation
        self.orientation
    }

    pub fn calculate_wandering_behavior(&mut self) -> f32 {
        let noise_frequency = 0.5;
        let noise_value = self
            .perlin
            .get([self.time_offset + get_time() * noise_frequency]);

        // Map the noise value (-1.0 to 1.0) to an angle (0 to 360 degrees)
        map_range(noise_value, -1.0, 1.0, 0.0, 360.0)
    }
}
