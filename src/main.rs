extern crate piston_window;
extern crate rand;

mod draw;
mod food;
mod nom;
mod simulation;

use piston_window::*;

use crate::simulation::Simulation;

const WINDOW_SIZE: [f64; 2] = [900.0, 600.0];

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Simulation", WINDOW_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut simulation = Simulation::new(4, 40);
    while let Some(event) = window.next() {
        // if let Some(Button::Keyboard(key)) = event.press_args() {
        //     simulation.key_pressed(key);
        // }
        window.draw_2d(&event, |c, g, _| {
            simulation.draw(&c, g);
        });

        // event.update(|arg| {
        //     simulation.update(arg.dt);
        // });
    }
}
