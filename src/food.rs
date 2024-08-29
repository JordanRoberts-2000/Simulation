use piston_window::types::Color;
use piston_window::{Context, G2d};

use crate::draw::{draw_circle, draw_ellipse};

const FOOD_COLOR: Color = [0.0, 1.0, 0.0, 1.0];
const MULTIPLIYER: u64 = 2;

pub struct Food {
    pub position: [u64; 2],
    pub size: u64,
}

impl Food {
    pub fn new(position: [u64; 2], size: u64) -> Food {
        return Food { position, size };
    }

    pub fn draw(position: [u64; 2], size: u64, c: &Context, g: &mut G2d) {
        draw_circle(
            [0.0, 0.2, 0.0, 1.0],
            [position[0], position[1]],
            size,
            &c,
            g,
        );
        for i in 0..8 {
            draw_ellipse(
                FOOD_COLOR,
                [position[0], position[1]],
                [size / 2, size * MULTIPLIYER],
                0 + (45 * i),
                &c,
                g,
            );
        }
    }
}
