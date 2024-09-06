use crate::simulation::Simulation;
use macroquad::prelude::*;

mod dev_tools;
mod entity_stats;
mod nom;
mod plants;
mod quadtree;
mod simulation;

fn window_conf() -> Conf {
    Conf {
        fullscreen: true,
        window_title: "Simulation".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulation = Simulation::new();
    loop {
        simulation.draw();
        simulation.update();
        next_frame().await;
    }
}
