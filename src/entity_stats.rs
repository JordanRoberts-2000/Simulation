use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nom::Nom;
use crate::utils::draw::draw_rounded_rectangle;

pub struct EntityStats {
    pub active: bool,
    current_nom: Option<Rc<RefCell<Nom>>>,
}

impl EntityStats {
    pub fn new() -> EntityStats {
        return EntityStats {
            active: true,
            current_nom: None,
        };
    }

    pub fn update(&mut self, noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>) {
        if is_mouse_button_pressed(MouseButton::Left) {
            // Get the mouse position
            let mouse_position = mouse_position(); // Returns (x, y) tuple

            // Borrow the noms vector
            let noms_borrow = noms.borrow();

            // Iterate through each Nom in the noms array
            let mut found_nom = false;
            for nom_ref in noms_borrow.iter() {
                let nom = nom_ref.borrow(); // Borrow the Nom

                // Get the Nom's position
                let nom_position = nom.position;

                // Calculate the distance between the mouse position and the nom's position
                let distance = ((mouse_position.0 - nom_position.x).powi(2)
                    + (mouse_position.1 - nom_position.y).powi(2))
                .sqrt();

                // Check if the distance is within 5 pixels
                if distance <= nom_ref.borrow().size / 2.0 {
                    // The Nom is within 5 pixels of the click, perform action (e.g., retrieve)
                    self.current_nom = Some(nom_ref.clone());
                    found_nom = true;
                    break;
                    // You can handle the retrieval logic here
                }
            }
            if !found_nom {
                self.current_nom = None;
            }
        }
    }

    pub fn draw(&self) {
        if !self.active || self.current_nom.is_none() {
            return;
        }
        if let Some(current_nom) = &self.current_nom {
            // Borrow and get the stats of the `Nom`
            let nom_stats = current_nom.borrow().get_stats();

            // Display stats
            let orientation = format!("Orientation: {}", nom_stats.orientation);
            let current_speed = format!("Current Speed: {}", nom_stats.current_speed);
            let max_speed = format!("Max Speed: {}", nom_stats.max_speed);
            let acceleration = format!("Acceleration: {}", nom_stats.acceleration);
            let velocity = format!(
                "Velocity: {}, {}",
                nom_stats.current_speed * nom_stats.orientation.to_radians().cos(),
                nom_stats.current_speed * nom_stats.orientation.to_radians().sin()
            );
            let border_width = 1.0;
            draw_rounded_rectangle(
                screen_width() - 330.0,
                screen_height() - 190.0,
                310.0,
                170.0,
                8.0,
                GRAY,
            );
            draw_rounded_rectangle(
                (screen_width() - 330.0) + border_width,
                (screen_height() - 190.0) + border_width,
                310.0 - (2.0 * border_width),
                170.0 - (2.0 * border_width),
                8.0,
                BLACK,
            );
            draw_text(
                &orientation,
                screen_width() - 300.,
                screen_height() - 160.,
                20.,
                WHITE,
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
}
