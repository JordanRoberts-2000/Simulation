use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

use crate::{
    nom::{Nom, NomVariant},
    utils::draw::draw_rounded_rectangle,
};

const GRID_GAP: f32 = 20.0;
const GRID_ITEM_SIZE: f32 = 100.0;
const GRID_ITEM_BORDER_WIDTH: f32 = 1.0;
const GRID_BORDER_COLOR: Color = GRAY;
const GRID_BORDER_COLOR_ACTIVE: Color = DARKBLUE;
const GRID_POSITION: Vec2 = vec2(30.0, 150.0);

pub struct NomSpawner {
    selected_index: (usize, usize),
    display_noms: Vec<Nom>,
}

impl NomSpawner {
    pub fn new() -> Self {
        Self {
            selected_index: (0, 0),
            display_noms: NomSpawner::display_noms(),
        }
    }

    pub fn draw(&self) {
        for i in 0..3 {
            for j in 0..3 {
                draw_rounded_rectangle(
                    GRID_POSITION.x + ((GRID_ITEM_SIZE + GRID_GAP) * i as f32),
                    GRID_POSITION.y + ((GRID_ITEM_SIZE + GRID_GAP) * j as f32),
                    GRID_ITEM_SIZE,
                    GRID_ITEM_SIZE,
                    10.0,
                    if self.selected_index == (i, j) {
                        GRID_BORDER_COLOR_ACTIVE
                    } else {
                        GRID_BORDER_COLOR
                    },
                );
                draw_rounded_rectangle(
                    GRID_POSITION.x
                        + ((GRID_ITEM_SIZE + GRID_GAP) * i as f32)
                        + GRID_ITEM_BORDER_WIDTH,
                    GRID_POSITION.y
                        + ((GRID_ITEM_SIZE + GRID_GAP) * j as f32)
                        + GRID_ITEM_BORDER_WIDTH,
                    GRID_ITEM_SIZE - GRID_ITEM_BORDER_WIDTH * 2.0,
                    GRID_ITEM_SIZE - GRID_ITEM_BORDER_WIDTH * 2.0,
                    10.0,
                    BLACK,
                );
                self.display_nom_draw((i, j));
            }
        }
    }

    pub fn handle_inputs(&mut self) {
        for i in 0..3 {
            for j in 0..3 {
                let x = GRID_POSITION.x + ((GRID_ITEM_SIZE + GRID_GAP) * i as f32);
                let y = GRID_POSITION.y + ((GRID_ITEM_SIZE + GRID_GAP) * j as f32);
                let rect = Rect::new(x, y, GRID_ITEM_SIZE, GRID_ITEM_SIZE);
                if is_mouse_button_pressed(MouseButton::Left)
                    && rect.contains(mouse_position().into())
                {
                    self.selected_index = (i, j);
                }
            }
        }
    }

    pub fn display_nom_draw(&self, index: (usize, usize)) {
        self.display_noms[0].draw(Rc::new(RefCell::new(false)));
        self.display_noms[1].draw(Rc::new(RefCell::new(false)));
        self.display_noms[2].draw(Rc::new(RefCell::new(false)));
        self.display_noms[3].draw(Rc::new(RefCell::new(false)));
        self.display_noms[4].draw(Rc::new(RefCell::new(false)));
        self.display_noms[5].draw(Rc::new(RefCell::new(false)));
        self.display_noms[6].draw(Rc::new(RefCell::new(false)));
        self.display_noms[7].draw(Rc::new(RefCell::new(false)));
        self.display_noms[8].draw(Rc::new(RefCell::new(false)));
    }

    pub fn display_noms() -> Vec<Nom> {
        let mut nom_array = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                let x = GRID_POSITION.x
                    + ((GRID_ITEM_SIZE + GRID_GAP) * i as f32)
                    + (GRID_ITEM_SIZE / 2.0);
                let y = GRID_POSITION.y
                    + ((GRID_ITEM_SIZE + GRID_GAP) * j as f32)
                    + (GRID_ITEM_SIZE / 2.0);
                let index: (usize, usize) = (i, j);
                let variant: NomVariant = match index {
                    (0, 0) => NomVariant::Default,
                    (0, 1) => NomVariant::RedMutation,
                    (0, 2) => NomVariant::Wendigo,
                    (1, 0) => NomVariant::GreenMutation,
                    (1, 1) => NomVariant::Hedgehog,
                    (1, 2) => NomVariant::Death,
                    (2, 0) => NomVariant::BlueMutation,
                    (2, 1) => NomVariant::Whale,
                    (2, 2) => NomVariant::Leviathan,
                    _ => panic!("Invalid index used in display_noms()"),
                };
                nom_array.push(Nom::display_new(vec2(x, y), variant));
            }
        }
        return nom_array;
    }
}
