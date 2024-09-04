use crate::simulation::{Simulation, SimulationType};
use macroquad::prelude::*;

mod dev_tools;
mod entity_stats;
mod nom;
mod plants;
mod simulation;

const SIMULATION_TYPE: SimulationType = SimulationType::Production;
const TESTING_VISUALS: bool = false;

fn window_conf() -> Conf {
    Conf {
        fullscreen: true, // Start the window in fullscreen mode
        window_title: "Simulation".to_owned(),
        ..Default::default() // Use the default values for the other options
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulation = Simulation::new(SIMULATION_TYPE, TESTING_VISUALS);
    loop {
        simulation.draw();
        simulation.key_pressed();
        simulation.update();
        // let center_x = screen_width() / 2.0;
        // let center_y = screen_height() / 2.0;

        // let plant_color: [f32; 3] = [0.0, 0.5, 0.5];
        // let plant_color_2: [f32; 3] = [0.0, 0.0, 0.85];

        // // Draw a series of circles to create a blur effect
        // draw_ellipse(
        //     center_x + 8.,
        //     center_y - 2.,
        //     40.0,
        //     35.0,
        //     25.0,
        //     Color::new(0.0, 0.3, 0.0, 0.15),
        // );

        // for i in 0..10 {
        //     let size = 1.0;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x, center_y, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.0;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x + 13.0, center_y + 6.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.5;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x + 14.0, center_y - 8.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.75;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x - 14.0, center_y - 8.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x - 20.0, center_y - 20.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x + 40.0, center_y - 20.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 0.75;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x + 44.0, center_y - 12.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.75;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x + 44.0, center_y + 20.0, radius, color);
        // }
        // // ==========================================================================================================

        // let center_x_2 = 50.0;
        // let center_y_2 = 50.0;
        // for i in 0..10 {
        //     let size = 1.0;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color_2[0], plant_color_2[1], plant_color_2[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_2, center_y_2, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.0;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color_2[0], plant_color_2[1], plant_color_2[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_2 + 13.0, center_y_2 + 6.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.5;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color_2[0], plant_color_2[1], plant_color_2[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_2 + 14.0, center_y_2 - 8.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.75;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color_2[0], plant_color_2[1], plant_color_2[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_2 - 14.0, center_y_2 - 8.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color_2[0], plant_color_2[1], plant_color_2[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_2 - 20.0, center_y_2 - 20.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color_2[0], plant_color_2[1], plant_color_2[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_2 + 40.0, center_y_2 - 20.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 0.75;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color_2[0], plant_color_2[1], plant_color_2[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_2 + 44.0, center_y_2 - 12.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.75;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color_2[0], plant_color_2[1], plant_color_2[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_2 + 44.0, center_y_2 + 20.0, radius, color);
        // }
        // // ==========================================================================================================

        // let center_x_3 = 300.0;
        // let center_y_3 = 550.0;
        // for i in 0..10 {
        //     let size = 1.0;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_3, center_y_3, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.0;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_3 + 13.0, center_y_3 + 6.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.5;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_3 + 14.0, center_y_3 - 8.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.75;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_3 - 14.0, center_y_3 - 8.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_3 - 20.0, center_y_3 - 20.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_3 + 40.0, center_y_3 - 20.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 0.75;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_3 + 44.0, center_y_3 - 12.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.75;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(center_x_3 + 44.0, center_y_3 + 20.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(1000., 306.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 0.75;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(1000., 300.0, radius, color);
        // }
        // for i in 0..10 {
        //     let size = 1.75;
        //     let radius = size + i as f32 * size;
        //     let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
        //     let color = Color::new(plant_color[0], plant_color[1], plant_color[2], alpha); // Semi-transparent color

        //     draw_circle(1150., 100.0, radius, color);
        // }

        next_frame().await;
    }
}
