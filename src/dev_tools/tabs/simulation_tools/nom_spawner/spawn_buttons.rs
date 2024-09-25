use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

use super::NomSpawner;
use crate::{
    nom::{Nom, NomVariant},
    utils::ui::button::Button,
};

impl NomSpawner {
    pub fn draw_spawn_buttons(&self) {
        for button in self.spawn_buttons.iter() {
            button.draw();
        }
    }

    pub fn update_spawn_buttons(&mut self) {
        for button in self.spawn_buttons.iter_mut() {
            button.update();
        }
    }

    pub fn create_spawn_buttons(
        noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
        selected: Rc<RefCell<NomVariant>>,
    ) -> Vec<Button> {
        let spawn_buttons_y: f32 = 635.0;

        let mut spawn_one_button = Button::new("Spawn nom");
        spawn_one_button.pos(20.0, spawn_buttons_y);
        spawn_one_button.padding(56.0, 4.0);
        spawn_one_button.on_click(spawn_nom(noms.clone(), selected.clone(), 1));

        let mut spawn_five_button = Button::new("+ 5");
        spawn_five_button.pos(161.0, spawn_buttons_y);
        spawn_five_button.on_click(spawn_nom(noms.clone(), selected.clone(), 5));

        let mut spawn_ten_button = Button::new("+ 10");
        spawn_ten_button.pos(212.0, spawn_buttons_y);
        spawn_ten_button.on_click(spawn_nom(noms.clone(), selected.clone(), 10));

        let mut spawn_twenty_button = Button::new("+ 20");
        spawn_twenty_button.pos(270.0, spawn_buttons_y);
        spawn_twenty_button.on_click(spawn_nom(noms.clone(), selected.clone(), 20));

        let mut spawn_fifty_button = Button::new("+ 50");
        spawn_fifty_button.pos(328.0, spawn_buttons_y);
        spawn_fifty_button.on_click(spawn_nom(noms.clone(), selected.clone(), 50));

        vec![
            spawn_one_button,
            spawn_five_button,
            spawn_ten_button,
            spawn_twenty_button,
            spawn_fifty_button,
        ]
    }
}

pub fn spawn_nom(
    noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
    selected: Rc<RefCell<NomVariant>>,
    amount: i32,
) -> impl FnMut() {
    let (noms, selected) = (noms.clone(), selected.clone());
    move || {
        let mut noms_mut = noms.borrow_mut();
        for _ in 0..amount {
            noms_mut.push(Rc::new(RefCell::new(Nom::spawn_random(
                selected.borrow().clone(),
            ))));
        }
    }
}
