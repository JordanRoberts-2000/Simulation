use piston_window::types::Color;
use piston_window::{Context, G2d};
use rand::{thread_rng, Rng};

use crate::config::WINDOW_SIZE;
use crate::draw::{draw_circle, draw_hollow_circle};

const NOM_SPAWN_SIZE: u64 = 6;
const NOM_COLOR: Color = [0.7, 0.0, 0.7, 1.0];
const NOM_BORDER_COLOR: Color = [0.4, 0.0, 0.4, 1.0];

pub struct Nom {
    pub size: u64,
    pub position: [u64; 2],
}

impl Nom {
    pub fn new() -> Nom {
        let mut rng = thread_rng();
        let border_spawn_offset = 8;
        let position_x =
            rng.gen_range(border_spawn_offset..(WINDOW_SIZE[0] as u64) - border_spawn_offset);
        let position_y =
            rng.gen_range(border_spawn_offset..(WINDOW_SIZE[1] as u64) - border_spawn_offset);
        return Nom {
            size: NOM_SPAWN_SIZE,
            position: [position_x, position_y],
        };
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
