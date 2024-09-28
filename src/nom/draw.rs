use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

use crate::simulation_state::SimulationState;

use super::{Nom, NomVariant};

#[derive(Clone)]
pub struct NomColors {
    body_color: Color,
    border_color: Color,
    glow_color: Color,
    mutation_color: Color,
}

impl Nom {
    pub fn draw(&self, state: Rc<RefCell<SimulationState>>) {
        if self.stats_active {
            self.draw_viewing_stats();
        }
        self.draw_spikes();
        self.draw_body();
        self.draw_glow();
        self.draw_mutation();
        if state.borrow().devtools.visuals.nom_wandering {
            self.draw_wandering_visuals();
        }
        if state.borrow().devtools.visuals.nom_detection_radius {
            self.draw_detection_radius();
        }
        if state.borrow().devtools.visuals.nom_orientation {
            self.draw_orientation_visuals();
        }
        if state.borrow().devtools.visuals.nom_target_orientation {
            self.draw_target_orientation_visuals();
        }
    }

    pub fn draw_display(&self) {
        self.draw_body();
        self.draw_glow();
        self.draw_mutation();
    }

    pub fn get_colors(variant: &NomVariant) -> NomColors {
        let body_color = match variant {
            NomVariant::Wendigo => Color::from_hex(0x450a0a),
            NomVariant::Death => BLACK,
            NomVariant::Leviathan => Color::new(0.2, 0.0, 0.2, 1.0),
            NomVariant::Whale => Color::new(0.2, 0.0, 0.2, 1.0),
            NomVariant::Shark => Color::from_hex(0x1e3a8a),
            _ => Color::new(0.2, 0.0, 0.2, 1.0),
        };

        let border_color = match variant {
            NomVariant::Wendigo => Color::from_hex(0x450a0a),
            NomVariant::Whale => Color::new(0.2, 0.0, 0.2, 1.0),
            NomVariant::Leviathan => Color::new(0.2, 0.0, 0.2, 1.0),
            NomVariant::Death => Color::from_hex(0x525252),
            NomVariant::Shark => Color::from_hex(0x3b82f6),
            _ => Color::new(0.9961, 0.0, 0.9961, 1.0), // Default border color
        };

        let glow_color = match variant {
            NomVariant::Default => Color::new(0.9961, 0.0, 0.9961, 1.0),
            NomVariant::BlueMutation => Color::new(0.231, 0.510, 0.965, 1.0),
            NomVariant::GreenMutation => GREEN,
            NomVariant::RedMutation => RED,
            NomVariant::Hedgehog => YELLOW,
            NomVariant::Wendigo => RED,
            NomVariant::Shark => Color::new(0.9961, 0.0, 0.9961, 1.0),
            NomVariant::Death => Color::from_hex(0x991b1b),
            _ => Color::new(0.0, 0.0, 0.0, 0.0), // Default glow color
        };

        let mutation_color = match variant {
            NomVariant::Default => Color::new(0.9961, 0.0, 0.9961, 1.0),
            NomVariant::BlueMutation => Color::new(0.231, 0.510, 0.965, 1.0),
            NomVariant::GreenMutation => GREEN,
            NomVariant::RedMutation => RED,
            NomVariant::Hedgehog => YELLOW,
            NomVariant::Wendigo => RED,
            NomVariant::Shark => RED,
            NomVariant::Death => Color::from_hex(0x991b1b),
            _ => Color::new(0.9961, 0.0, 0.9961, 1.0),
        };
        NomColors {
            body_color,
            border_color,
            glow_color,
            mutation_color,
        }
    }

    pub fn draw_spikes(&self) {
        if self.spike_amount == 0 {
            return;
        };
        let angles = vec![
            std::f32::consts::FRAC_PI_8,
            std::f32::consts::FRAC_PI_8 * 2.0,
            std::f32::consts::PI - std::f32::consts::FRAC_PI_8 * 3.0,
            std::f32::consts::FRAC_PI_8 * 4.0 + std::f32::consts::PI,
            std::f32::consts::FRAC_PI_8 * 6.0 + std::f32::consts::PI,
            std::f32::consts::PI + std::f32::consts::FRAC_PI_8,
        ];
        for angle in angles.iter().take(self.spike_amount as usize) {
            let x_pos = self.position[0] + ((self.size / 2.0) - 3.0) * angle.cos();
            let y_pos = self.position[1] + ((self.size / 2.0) - 3.0) * angle.sin();

            draw_ellipse(x_pos, y_pos, 18.0, 2.0, angle.to_degrees(), LIGHTGRAY);

            draw_ellipse(
                x_pos,
                y_pos,
                6.0,
                6.0,
                angle.to_degrees(),
                self.colors.border_color,
            );
        }
    }

    fn draw_body(&self) {
        draw_circle(
            self.position[0],
            self.position[1],
            self.size / 2.0,
            if self.size >= 4.0 {
                self.colors.border_color
            } else {
                self.colors.body_color
            },
        );
        if self.has_twin {
            draw_circle(
                self.position[0] + self.size / 2.5,
                self.position[1] + self.size / 2.5,
                self.size / 2.0,
                if self.size >= 4.0 {
                    self.colors.border_color
                } else {
                    self.colors.body_color
                },
            );
        }
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
                        (self.size / 2.0) - if self.size > 18.0 { 2.0 } else { 1.5 },
                        self.colors.body_color,
                    );
                    if self.has_twin {
                        draw_circle(
                            self.position[0] + self.size / 2.5,
                            self.position[1] + self.size / 2.5,
                            (self.size / 2.0) - if self.size > 18.0 { 2.0 } else { 1.5 },
                            self.colors.body_color,
                        );
                    }
                }
            }
        }
    }

    pub fn draw_glow(&self) {
        if self.variant == NomVariant::Leviathan || self.variant == NomVariant::Whale {
            return;
        }
        let offset = match self.variant {
            NomVariant::Wendigo | NomVariant::Hedgehog => 2.0,
            _ => 4.0,
        };
        let transparancy = match self.variant {
            NomVariant::Hedgehog => 0.3,
            _ => 0.2,
        };

        draw_circle(
            self.position[0],
            self.position[1],
            ((self.size / 2.0) - offset) - if self.size > 18.0 { 2.0 } else { 1.5 },
            Color::new(
                self.colors.glow_color.r,
                self.colors.glow_color.g,
                self.colors.glow_color.b,
                transparancy,
            ),
        );
        if self.has_twin {
            draw_circle(
                self.position[0] + self.size / 2.5,
                self.position[1] + self.size / 2.5,
                ((self.size / 2.0) - offset) - if self.size > 18.0 { 2.0 } else { 1.5 },
                Color::new(
                    self.colors.glow_color.r,
                    self.colors.glow_color.g,
                    self.colors.glow_color.b,
                    transparancy,
                ),
            );
        }
    }

    fn draw_mutation(&self) {
        if self.variant == NomVariant::Leviathan || self.variant == NomVariant::Whale {
            return;
        }
        if self.mutation_variant == 0 {
            self.draw_mutation_bubble(self.rotate_point(vec2(5.0, -1.0)), false);
            self.draw_mutation_bubble(self.rotate_point(vec2(-2.0, -3.0)), true);
            self.draw_mutation_bubble(self.rotate_point(vec2(-4.0, 3.0)), false);
            self.draw_mutation_bubble(self.rotate_point(vec2(3.0, 4.0)), true);
        }
        if self.mutation_variant == 1 {
            self.draw_mutation_bubble(self.rotate_point(vec2(0.0, -3.0)), true);
            self.draw_mutation_bubble(self.rotate_point(vec2(-3.0, 3.0)), false);
            self.draw_mutation_bubble(self.rotate_point(vec2(3.0, 3.0)), false);
        }
        if self.mutation_variant == 2 {
            self.draw_mutation_bubble(self.rotate_point(vec2(1.0, -2.0)), true);
            self.draw_mutation_bubble(self.rotate_point(vec2(-2.0, 3.0)), false);
        }
    }

    fn draw_mutation_bubble(&self, position: Vec2, large: bool) {
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
            if large && self.size >= 18.0 && self.variant != NomVariant::Hedgehog {
                2.0
            } else {
                1.0
            },
            self.colors.mutation_color,
        );
        if self.has_twin {
            draw_circle(
                self.position[0]
                    + self.size / 2.5
                    + ((self.size.clamp(start_size, finish_size) - start_size)
                        / (finish_size - start_size))
                        * position.x,
                self.position[1]
                    + self.size / 2.5
                    + ((self.size.clamp(start_size, finish_size) - start_size)
                        / (finish_size - start_size))
                        * position.y,
                if large && self.size >= 18.0 { 2.0 } else { 1.0 },
                self.colors.mutation_color,
            );
        }
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
        )
    }

    pub fn draw_wandering_visuals(&self) {
        let look_ahead_distance = self.look_ahead_distance;
        let wander_radius = self.look_ahead_size / 2.0;
        let look_ahead_position = Vec2::new(
            self.position.x + self.orientation.cos() * look_ahead_distance,
            self.position.y + self.orientation.sin() * look_ahead_distance,
        );
        draw_circle_lines(
            look_ahead_position.x,
            look_ahead_position.y,
            wander_radius,
            2.0,
            GREEN,
        );
        draw_circle(self.look_ahead_target.x, self.look_ahead_target.y, 5.0, RED);
        draw_line(
            self.position.x,
            self.position.y,
            self.look_ahead_target.x,
            self.look_ahead_target.y,
            2.0,
            RED,
        );
    }

    pub fn draw_orientation_visuals(&self) {
        let line_length = 30.0;
        let x2 = self.position.x + self.orientation.cos() * line_length;
        let y2 = self.position.y + self.orientation.sin() * line_length;
        draw_line(self.position.x, self.position.y, x2, y2, 2.0, RED);
    }

    pub fn draw_target_orientation_visuals(&self) {
        let line_length = 30.0;
        let x2 = self.position.x + self.target_orientation.cos() * line_length;
        let y2 = self.position.y + self.target_orientation.sin() * line_length;
        draw_line(self.position.x, self.position.y, x2, y2, 2.0, BLUE);
    }

    pub fn draw_detection_radius(&self) {
        draw_circle_lines(self.position.x, self.position.y, 200.0, 2.0, DARKGRAY);
    }
}
