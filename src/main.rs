extern crate piston_window;
extern crate rand;

mod draw;

use piston_window::types::Color;
use piston_window::*;

use crate::draw::{draw_detection_range, draw_food, draw_nom};

const WINDOW_SIZE: [f64; 2] = [800.0, 800.0];
const BACKGROUND_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Simulation", WINDOW_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        // if let Some(Button::Keyboard(key)) = event.press_args() {
        //     game.key_pressed(key);
        // }
        window.draw_2d(&event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            draw_food([0.0, 1.0, 0.0, 1.0], 100, 100, &c, g);
            draw_detection_range(300.0, 300.0, 80.0, [0.0, 0.0, 1.0, 1.0], &c, g);
            draw_nom([300, 300], 22, &c, g);
            // game.draw(&c, g);
        });

        // event.update(|arg| {
        //     game.update(arg.dt);
        // });
    }
}
