use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nom::{Nom, NomVariant};
use crate::utils::draw::draw_rounded_rectangle;
use crate::utils::ui::button::Button;

pub struct EntityStats {
    pub active: bool,
    current_nom: Rc<RefCell<Option<Rc<RefCell<Nom>>>>>,
    display_nom: Nom,
    kill_button: Button,
}

impl EntityStats {
    pub fn new(selected_nom: Rc<RefCell<Option<Rc<RefCell<Nom>>>>>) -> EntityStats {
        let mut kill_button = Button::new("Kill");
        kill_button.pos(screen_width() - 88.0, screen_height() - 114.0);
        kill_button.on_click(|| println!("kill nom"));
        kill_button.padding(4.0, 2.0);
        return EntityStats {
            active: true,
            current_nom: selected_nom.clone(),
            display_nom: Nom::display_new(
                vec2(screen_width() - 60.0, screen_height() - 148.0),
                NomVariant::Default,
            ),
            kill_button,
        };
    }

    pub fn update(&mut self, noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>, devtools_active: &bool) {
        if self.active {
            self.kill_button.update();
        }
        self.check_click(noms, devtools_active);
    }

    pub fn check_click(
        &mut self,
        noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
        devtools_active: &bool,
    ) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();
            let rect_x = screen_width() - 330.0;
            let rect_y = screen_height() - 182.0;
            let rect_width = 310.0;
            let rect_height = 160.0;

            if self.active {
                if mouse_position.0 >= rect_x
                    && mouse_position.0 <= rect_x + rect_width
                    && mouse_position.1 >= rect_y
                    && mouse_position.1 <= rect_y + rect_height
                {
                    return;
                }

                if *devtools_active {
                    let dev_rect_x = 0.0;
                    let dev_rect_y = 0.0;
                    let dev_rect_width = 400.0;
                    let dev_rect_height = screen_height();
                    if mouse_position.0 >= dev_rect_x
                        && mouse_position.0 <= dev_rect_x + dev_rect_width
                        && mouse_position.1 >= dev_rect_y
                        && mouse_position.1 <= dev_rect_y + dev_rect_height
                    {
                        return;
                    }
                }
            }

            let noms_borrow = noms.borrow();
            let mut found_nom = false;
            for nom_ref in noms_borrow.iter() {
                let distance;
                {
                    let nom = nom_ref.borrow();
                    let nom_position = nom.position;
                    distance = ((mouse_position.0 - nom_position.x).powi(2)
                        + (mouse_position.1 - nom_position.y).powi(2))
                    .sqrt();

                    if distance > nom.size / 2.0 {
                        continue;
                    }
                }

                // Show stats for the clicked nom
                nom_ref.borrow_mut().show_stats();
                found_nom = true;

                // Borrow current_nom and update it
                let mut current_nom_borrow = self.current_nom.borrow_mut();
                if let Some(current_nom) = &*current_nom_borrow {
                    if !Rc::ptr_eq(current_nom, nom_ref) {
                        current_nom.borrow_mut().hide_stats();
                    }
                }

                // Update current_nom
                *current_nom_borrow = Some(nom_ref.clone());

                let new_variant = nom_ref.borrow().get_stats().variant;
                let size = match new_variant {
                    NomVariant::Leviathan => 44.0,
                    NomVariant::Whale => 44.0,
                    _ => 32.0,
                };
                self.display_nom.set_size(size);
                self.display_nom.set_variant(new_variant);

                break;
            }

            if !found_nom {
                let mut current_nom_borrow = self.current_nom.borrow_mut();
                if let Some(current_nom) = &*current_nom_borrow {
                    current_nom.borrow_mut().hide_stats();
                }
                *current_nom_borrow = None;
            }
        }
    }

    pub fn draw(&self) {
        if !self.active || self.current_nom.borrow().is_none() {
            return;
        }

        if let Some(current_nom) = self.current_nom.borrow().as_ref() {
            let nom_stats = current_nom.borrow().get_stats();
            let name_string = match nom_stats.variant {
                NomVariant::Default => "Nom".to_string(),
                NomVariant::BlueMutation => "Blue Mutation".to_string(),
                NomVariant::GreenMutation => "Green Mutation".to_string(),
                NomVariant::RedMutation => "Red Mutation".to_string(),
                _ => format!("{:?}", nom_stats.variant),
            };
            let state = format!("State: {:?}", nom_stats.state);
            let age = format!("Age: {:?}", (get_time() - nom_stats.age) as u64);
            let border_width = 1.0;
            draw_rounded_rectangle(
                screen_width() - 330.0,
                screen_height() - 182.0,
                310.0,
                160.0,
                8.0,
                GRAY,
            );
            draw_rounded_rectangle(
                (screen_width() - 330.0) + border_width,
                (screen_height() - 182.0) + border_width,
                310.0 - (2.0 * border_width),
                160.0 - (2.0 * border_width),
                8.0,
                BLACK,
            );
            self.display_nom.draw_display();
            self.kill_button.draw();
            draw_line(
                screen_width() - 312.0,
                screen_height() - 126.0,
                screen_width() - 90.0,
                screen_height() - 126.0,
                1.0,
                GRAY,
            );
            draw_text(
                &name_string,
                screen_width() - 312.,
                screen_height() - 140.,
                28.,
                WHITE,
            );
            draw_text(
                &state,
                screen_width() - 312.,
                screen_height() - 100.,
                20.,
                WHITE,
            );
            draw_text(
                "Hunger:",
                screen_width() - 312.,
                screen_height() - 70.,
                20.,
                WHITE,
            );
            let bar_border_width = 2.0;
            draw_rounded_rectangle(
                screen_width() - 238.0,
                screen_height() - 78.0,
                192.0,
                8.0,
                2.0,
                DARKGREEN,
            );
            draw_rounded_rectangle(
                (screen_width() - 238.0) + bar_border_width,
                (screen_height() - 78.0) + bar_border_width,
                120.0 - (2.0 * bar_border_width),
                8.0 - (2.0 * bar_border_width),
                2.0,
                GREEN,
            );

            draw_text(
                "Age:",
                screen_width() - 312.,
                screen_height() - 70. + 30.0,
                20.,
                WHITE,
            );
            draw_rounded_rectangle(
                screen_width() - 238.0,
                screen_height() - 78.0 + 30.,
                192.0,
                8.0,
                2.0,
                DARKBLUE,
            );
            draw_rounded_rectangle(
                (screen_width() - 238.0) + bar_border_width,
                (screen_height() - 78.0 + 30.) + bar_border_width,
                120.0 - (2.0 * bar_border_width),
                8.0 - (2.0 * bar_border_width),
                2.0,
                BLUE,
            );
        }
    }
}
