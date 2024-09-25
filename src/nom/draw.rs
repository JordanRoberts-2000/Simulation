use macroquad::prelude::*;

use super::{Nom, NomVariant};

impl Nom {
    pub fn draw(&self) {
        if self.stats_active {
            self.draw_viewing_stats();
        }
        self.draw_body();
        self.draw_mutation();
        // if *testing_visuals.borrow() {
        //     self.draw_testing_visuals();
        // }
    }

    fn draw_body(&self) {
        let (color, border_color) = match self.variant {
            NomVariant::Wendigo => (Color::from_hex(0x450a0a), Color::from_hex(0xdc2626)),
            NomVariant::Death => (BLACK, Color::from_hex(0x525252)),
            NomVariant::Leviathan => (
                Color::new(0.2, 0.0, 0.2, 1.0),
                Color::new(0.2, 0.0, 0.2, 1.0),
            ),
            NomVariant::Whale => (
                Color::new(0.2, 0.0, 0.2, 1.0),
                Color::new(0.2, 0.0, 0.2, 1.0),
            ),
            NomVariant::Shark => (Color::from_hex(0x1e3a8a), Color::from_hex(0x3b82f6)),
            _ => (
                Color::new(0.2, 0.0, 0.2, 1.0),
                Color::new(0.9961, 0.0, 0.9961, 1.0),
            ),
        };
        draw_circle(
            self.position[0],
            self.position[1],
            self.size / 2.0,
            if self.size >= 4.0 {
                border_color
            } else {
                color
            },
        );
        match self.variant {
            NomVariant::Whale => {
                for i in 0..10 {
                    let size = self.size / 20.0;
                    let radius = size + (i as f32 * size);
                    let alpha = 0.05 * i as f32; // Decrease alpha for outer circles
                    let color = Color::new(0.2764706, 0.2, 0.91764706, alpha); // Semi-transparent color

                    draw_circle(self.position[0], self.position[1], radius, color);
                }
            }
            NomVariant::Leviathan => {
                for i in 0..20 {
                    let size = self.size / 20.0;
                    let radius = size + i as f32 * size;
                    let alpha = 0.05 * (10 - i) as f32; // Decrease alpha for outer circles
                    let color = Color::new(0.9764706, 0.2, 0.21764706, alpha); // Semi-transparent color

                    draw_circle(self.position[0], self.position[1], radius, color);
                }
            }
            _ => {
                if self.size >= 4.0 {
                    draw_circle(
                        self.position[0],
                        self.position[1],
                        (self.size / 2.0) - if self.size >= 14.0 { 1.5 } else { 1.0 },
                        color,
                    );
                }
            }
        }
    }

    fn draw_mutation(&self) {
        if self.variant == NomVariant::Leviathan || self.variant == NomVariant::Whale {
            return;
        }
        let mutation_color = match self.variant {
            NomVariant::Default => Color::new(0.9961, 0.0, 0.9961, 1.0),
            NomVariant::BlueMutation => Color::new(0.231, 0.510, 0.965, 1.0),
            NomVariant::GreenMutation => GREEN,
            NomVariant::RedMutation => RED,
            NomVariant::Hedgehog => YELLOW,
            NomVariant::Wendigo => RED,
            NomVariant::Shark => Color::from_hex(0xf87171),
            NomVariant::Death => Color::from_hex(0x991b1b),
            _ => Color::new(0.9961, 0.0, 0.9961, 1.0),
        };
        if self.mutation_variant == 0 {
            self.draw_mutation_bubble(self.rotate_point(vec2(5.0, -1.0)), false, mutation_color);
            self.draw_mutation_bubble(self.rotate_point(vec2(-2.0, -3.0)), true, mutation_color);
            self.draw_mutation_bubble(self.rotate_point(vec2(-3.0, 2.0)), false, mutation_color);
            self.draw_mutation_bubble(self.rotate_point(vec2(3.0, 4.0)), false, mutation_color);
        }
        if self.mutation_variant == 1 {
            self.draw_mutation_bubble(self.rotate_point(vec2(0.0, -3.0)), true, mutation_color);
            self.draw_mutation_bubble(self.rotate_point(vec2(-3.0, 2.0)), false, mutation_color);
            self.draw_mutation_bubble(self.rotate_point(vec2(3.0, 1.0)), false, mutation_color);
        }
        if self.mutation_variant == 2 {
            self.draw_mutation_bubble(self.rotate_point(vec2(1.0, -2.0)), true, mutation_color);
            self.draw_mutation_bubble(self.rotate_point(vec2(-2.0, 3.0)), false, mutation_color);
        }
    }

    fn draw_mutation_bubble(&self, position: Vec2, large: bool, mutation_color: Color) {
        let start_size: f32 = 8.0;
        let finish_size: f32 = 18.0;
        draw_circle(
            self.position[0]
                + ((self.size.clamp(start_size, finish_size) - start_size)
                    / (finish_size - start_size))
                    * position.x,
            self.position[1]
                + ((self.size.clamp(start_size, finish_size) - start_size)
                    / (finish_size - start_size))
                    * position.y,
            if large && self.size >= 18.0 { 2.0 } else { 1.0 },
            mutation_color,
        );
    }

    fn rotate_point(&self, point: Vec2) -> Vec2 {
        let cos_theta = self.orientation.cos();
        let sin_theta = self.orientation.sin();
        Vec2::new(
            point.x * cos_theta - point.y * sin_theta, // New x after rotation
            point.x * sin_theta + point.y * cos_theta, // New y after rotation
        )
    }

    pub fn draw_viewing_stats(&self) {
        draw_circle(
            self.position[0],
            self.position[1],
            (self.size / 2.0) + 3.0,
            Color::new(0.0, 0.0, 0.9882, 0.3),
        );
        draw_circle(
            self.position[0],
            self.position[1],
            (self.size / 2.0) + 7.0,
            Color::new(0.0, 0.0, 0.9882, 0.3),
        );
    }

    pub fn draw_testing_visuals(&self) {
        // Detection radius
        draw_circle_lines(self.position.x, self.position.y, 200.0, 2.0, DARKGRAY);
        // Orientation
        {
            let line_length = 30.0;
            let x2 = self.position.x + self.orientation.cos() * line_length;
            let y2 = self.position.y + self.orientation.sin() * line_length;
            draw_line(self.position.x, self.position.y, x2, y2, 2.0, RED);
        }
        // Target orientation
        {
            let line_length = 30.0;
            let x2 = self.position.x + self.target_orientation.cos() * line_length;
            let y2 = self.position.y + self.target_orientation.sin() * line_length;
            draw_line(self.position.x, self.position.y, x2, y2, 2.0, BLUE);
        }
        // Wander steering
        let look_ahead_distance = self.look_ahead_distance;
        let wander_radius = self.look_ahead_size / 2.0;
        let look_ahead_position = Vec2::new(
            self.position.x + self.orientation.cos() * look_ahead_distance,
            self.position.y + self.orientation.sin() * look_ahead_distance,
        );
        // draw_circle_lines(
        //     look_ahead_position.x,
        //     look_ahead_position.y,
        //     wander_radius,
        //     2.0,
        //     GREEN,
        // );
        // draw_circle(self.look_ahead_target.x, self.look_ahead_target.y, 5.0, RED);
        // draw_line(
        //     self.position.x,
        //     self.position.y,
        //     self.look_ahead_target.x,
        //     self.look_ahead_target.y,
        //     2.0,
        //     RED,
        // );
    }
}
