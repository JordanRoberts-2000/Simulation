use piston_window::types::Color;
use piston_window::{Context, G2d};

use crate::draw::{draw_circle, draw_hollow_circle};

const NOM_COLOR: Color = [0.7, 0.0, 0.7, 1.0];
const NOM_BORDER_COLOR: Color = [0.4, 0.0, 0.4, 1.0];

pub struct Nom {
    pub size: u64,
    pub position: [u64; 2],
}

impl Nom {
    pub fn new(position: [u64; 2]) -> Nom {
        return Nom { size: 6, position };
    }

    pub fn draw(position: [u64; 2], size: u64, c: &Context, g: &mut G2d) {
        draw_circle(
            if size > 3 {
                NOM_COLOR
            } else {
                NOM_BORDER_COLOR
            },
            position,
            size,
            &c,
            g,
        );
        if size > 3 {
            draw_hollow_circle(
                &c,
                g,
                NOM_BORDER_COLOR,
                [position[0] as f64, position[1] as f64],
                (size / 2) as f64,
                100,
            );
        }
    }
}
