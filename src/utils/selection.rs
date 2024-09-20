use macroquad::prelude::*;

pub struct Selection {
    index: u32,
    position: Option<Vec2>,
}

impl Selection {
    pub fn new() -> Self {
        Self {
            index: 0,
            position: None,
        }
    }

    pub fn draw(&self) {
        if let Some(pos) = self.position {}
    }

    pub fn update(&self) {
        if let Some(pos) = self.position {}
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.position = Some(vec2(x, y));
    }
}