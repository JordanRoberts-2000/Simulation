use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::{Nom, NomVariant};

impl Nom {
    pub fn draw(&self, testing_visuals: Rc<RefCell<bool>>) {
        self.draw_body();
        self.draw_mutation();

        // Draw the direction the object is facing (orientation line)
        if *testing_visuals.borrow() {
            let line_length = 30.0;
            let x2 = self.position.x + self.orientation.to_radians().cos() * line_length;
            let y2 = self.position.y + self.orientation.to_radians().sin() * line_length;
            draw_line(self.position.x, self.position.y, x2, y2, 2.0, RED);
        }
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
            NomVariant::Death => Color::from_hex(0x991b1b),
            _ => Color::new(0.9961, 0.0, 0.9961, 1.0),
        };
        if self.mutation_variant == 0 {
            self.draw_mutation_bubble(vec2(5.0, -1.0), false, mutation_color);
            self.draw_mutation_bubble(vec2(-2.0, -3.0), true, mutation_color);
            self.draw_mutation_bubble(vec2(-3.0, 2.0), false, mutation_color);
            self.draw_mutation_bubble(vec2(3.0, 4.0), false, mutation_color);
        }
        if self.mutation_variant == 1 {
            self.draw_mutation_bubble(vec2(0.0, -3.0), true, mutation_color);
            self.draw_mutation_bubble(vec2(-3.0, 2.0), false, mutation_color);
            self.draw_mutation_bubble(vec2(3.0, 1.0), false, mutation_color);
        }
        if self.mutation_variant == 2 {
            self.draw_mutation_bubble(vec2(1.0, -2.0), true, mutation_color);
            self.draw_mutation_bubble(vec2(-2.0, 3.0), false, mutation_color);
        }
    }

    fn draw_mutation_bubble(&self, position: Vec2, large: bool, mutation_color: Color) {
        let start_size: f32 = 8.0;
        let finish_size: f32 = 18.0;
        draw_circle(
            self.position[0]
                + (((self.size.clamp(start_size, finish_size) - start_size)
                    / (finish_size - start_size))
                    * position.x)
                    .round(),
            self.position[1]
                + (((self.size.clamp(start_size, finish_size) - start_size)
                    / (finish_size - start_size))
                    * position.y)
                    .round(),
            if large && self.size >= 18.0 { 2.0 } else { 1.0 },
            mutation_color,
        );
    }
}
