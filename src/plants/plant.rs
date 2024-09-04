use macroquad::prelude::*;

const PLANT_COLOR: [f32; 3] = [0.0, 0.7, 0.0];

pub enum PlantSize {
    Sm,
    Md,
    Lg,
}

impl PlantSize {
    fn size_value(&self) -> f32 {
        match self {
            PlantSize::Sm => 0.75,
            PlantSize::Md => 1.25,
            PlantSize::Lg => 1.75,
        }
    }
}

pub struct Plant {
    position: [f32; 2],
    size: PlantSize,
}

impl Plant {
    pub fn new(position: [f32; 2], size: PlantSize) -> Plant {
        return Plant { position, size };
    }
    pub fn draw(&self) {
        for i in 0..10 {
            let size = 1.0;
            let radius = size + i as f32 * size;
            let alpha = 0.1 * (5 - i) as f32; // Decrease alpha for outer circles
            let color = Color::new(PLANT_COLOR[0], PLANT_COLOR[1], PLANT_COLOR[2], alpha); // Semi-transparent color

            draw_circle(self.position[0], self.position[1], radius, color);
        }
    }
}
