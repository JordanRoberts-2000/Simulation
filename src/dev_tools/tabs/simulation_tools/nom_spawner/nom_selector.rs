use ::macroquad::prelude::*;

use crate::{
    nom::{Nom, NomVariant},
    utils::draw::draw_rounded_rectangle,
};

use super::NomSpawner;

const GRID_GAP: f32 = 20.0;
const GRID_ITEM_SIZE: f32 = 90.0;
const GRID_ITEM_BORDER_WIDTH: f32 = 2.0;
const GRID_BORDER_COLOR: Color = GRAY;
const GRID_BORDER_COLOR_ACTIVE: Color = DARKBLUE;
const GRID_POSITION: Vec2 = vec2(40.0, 180.0);
const GRID_POSITION_TO_VARIANT: [(usize, usize, NomVariant); 9] = [
    (0, 0, NomVariant::Default),
    (1, 0, NomVariant::GreenMutation),
    (2, 0, NomVariant::BlueMutation),
    (0, 1, NomVariant::RedMutation),
    (1, 1, NomVariant::Hedgehog),
    (2, 1, NomVariant::Shark),
    (0, 2, NomVariant::Wendigo),
    (1, 2, NomVariant::Death),
    (2, 2, NomVariant::Whale),
];

impl NomSpawner {
    pub fn draw_nom_selector(&self) {
        self.draw_selection_grid();
        for nom in self.display_noms.iter() {
            nom.draw();
        }
    }

    pub fn update_nom_selector(&mut self) {
        for (i, j, variant) in GRID_POSITION_TO_VARIANT.iter() {
            let x = GRID_POSITION.x + ((GRID_ITEM_SIZE + GRID_GAP) * *i as f32);
            let y = GRID_POSITION.y + ((GRID_ITEM_SIZE + GRID_GAP) * *j as f32);
            let rect = Rect::new(x, y, GRID_ITEM_SIZE, GRID_ITEM_SIZE);
            if is_mouse_button_pressed(MouseButton::Left) && rect.contains(mouse_position().into())
            {
                let mut selected_variant = self.nom_variant_selected.borrow_mut();
                *selected_variant = variant.clone();
            }
        }
    }

    pub fn draw_selection_grid(&self) {
        for (i, j, variant) in GRID_POSITION_TO_VARIANT.iter() {
            draw_rounded_rectangle(
                GRID_POSITION.x + ((GRID_ITEM_SIZE + GRID_GAP) * *i as f32),
                GRID_POSITION.y + ((GRID_ITEM_SIZE + GRID_GAP) * *j as f32),
                GRID_ITEM_SIZE,
                GRID_ITEM_SIZE,
                10.0,
                if *self.nom_variant_selected.borrow() == *variant {
                    GRID_BORDER_COLOR_ACTIVE
                } else {
                    GRID_BORDER_COLOR
                },
            );
            draw_rounded_rectangle(
                GRID_POSITION.x
                    + ((GRID_ITEM_SIZE + GRID_GAP) * *i as f32)
                    + GRID_ITEM_BORDER_WIDTH,
                GRID_POSITION.y
                    + ((GRID_ITEM_SIZE + GRID_GAP) * *j as f32)
                    + GRID_ITEM_BORDER_WIDTH,
                GRID_ITEM_SIZE - GRID_ITEM_BORDER_WIDTH * 2.0,
                GRID_ITEM_SIZE - GRID_ITEM_BORDER_WIDTH * 2.0,
                10.0,
                BLACK,
            );
        }
    }

    pub fn create_display_noms() -> Vec<Nom> {
        GRID_POSITION_TO_VARIANT
            .iter()
            .map(|(i, j, variant)| {
                let x = GRID_POSITION.x
                    + ((GRID_ITEM_SIZE + GRID_GAP) * *i as f32)
                    + (GRID_ITEM_SIZE / 2.0);
                let y = GRID_POSITION.y
                    + ((GRID_ITEM_SIZE + GRID_GAP) * *j as f32)
                    + (GRID_ITEM_SIZE / 2.0);
                Nom::display_new(vec2(x, y), variant.clone())
            })
            .collect()
    }
}
