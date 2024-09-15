use super::Nom;
use macroquad::prelude::*;

impl Nom {
    pub fn check_collision(&self, nom: &Nom) -> bool {
        // if !self.bounding_box_collision(nom) {
        //     return false;
        // }
        return self.circle_collision(nom);
    }

    fn bounding_box_collision(&self, nom: &Nom) -> bool {
        !(self.position.x + self.size / 2.0 < nom.position.x - nom.size / 2.0 || // Right of self.position is left of nom.position
    self.position.x - self.size / 2.0 > nom.position.x + nom.size / 2.0 || // Left of self.position is right of nom.position
    self.position.y + self.size / 2.0 < nom.position.y - nom.size / 2.0 || // Bottom of self.position is above nom.position
    self.position.y - self.size / 2.0 > nom.position.y + nom.size / 2.0) // Top of circle1 is below circle2
    }

    fn circle_collision(&self, nom: &Nom) -> bool {
        let dx = nom.position.x - self.position.x;
        let dy = nom.position.y - self.position.y;
        let distance_squared = dx * dx + dy * dy;
        let radii_sum = self.size / 2.0 + nom.size / 2.0;
        return distance_squared <= radii_sum * radii_sum;
    }

    pub fn clamp_position_to_screen(&mut self) {
        let radius = self.size / 2.0;
        self.position.x = self.position.x.clamp(radius, screen_width() - radius);
        self.position.y = self.position.y.clamp(radius, screen_height() - radius);
    }
}
