use macroquad::prelude::*;

use crate::nom::{self, Nom};

pub struct EntityStats {
    pub active: bool,
}

impl EntityStats {
    pub fn new() -> EntityStats {
        return EntityStats { active: true };
    }

    pub fn draw(&self, noms: &[Nom]) {
        let nom_stats = noms[0].get_stats();
        let current_speed = format!("Current Speed: {}", nom_stats.current_speed);
        let max_speed = format!("Max Speed: {}", nom_stats.max_speed);
        let acceleration = format!("Acceleration: {}", nom_stats.acceleration);
        let velocity = format!(
            "velocity: {}, {}",
            nom_stats.current_speed * nom_stats.orientation.to_radians().cos(),
            nom_stats.current_speed * nom_stats.orientation.to_radians().sin()
        );
        draw_text(
            &velocity,
            screen_width() - 300.,
            screen_height() - 130.,
            20.,
            WHITE,
        );
        draw_text(
            &current_speed,
            screen_width() - 300.,
            screen_height() - 100.,
            20.,
            WHITE,
        );
        draw_text(
            &max_speed,
            screen_width() - 300.,
            screen_height() - 70.,
            20.,
            WHITE,
        );
        draw_text(
            &acceleration,
            screen_width() - 300.,
            screen_height() - 40.,
            20.,
            WHITE,
        );
    }
}
