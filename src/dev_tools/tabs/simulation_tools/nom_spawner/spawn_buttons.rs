use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

use super::{spawn_settings::NomTwins, NomSpawner};
use crate::{
    nom::{Nom, NomVariant},
    simulation_state::SimulationState,
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
        state: Rc<RefCell<SimulationState>>,
        selected: Rc<RefCell<NomVariant>>,
    ) -> Vec<Button> {
        let spawn_buttons_y: f32 = 635.0;

        let mut spawn_one_button = Button::new("Spawn nom");
        spawn_one_button.pos(20.0, spawn_buttons_y);
        spawn_one_button.padding(56.0, 4.0);
        spawn_one_button.on_click(spawn_nom(state.clone(), selected.clone(), 1));

        let mut spawn_five_button = Button::new("+ 5");
        spawn_five_button.pos(161.0, spawn_buttons_y);
        spawn_five_button.on_click(spawn_nom(state.clone(), selected.clone(), 5));

        let mut spawn_ten_button = Button::new("+ 10");
        spawn_ten_button.pos(212.0, spawn_buttons_y);
        spawn_ten_button.on_click(spawn_nom(state.clone(), selected.clone(), 10));

        let mut spawn_twenty_button = Button::new("+ 20");
        spawn_twenty_button.pos(270.0, spawn_buttons_y);
        spawn_twenty_button.on_click(spawn_nom(state.clone(), selected.clone(), 20));

        let mut spawn_fifty_button = Button::new("+ 50");
        spawn_fifty_button.pos(328.0, spawn_buttons_y);
        spawn_fifty_button.on_click(spawn_nom(state.clone(), selected.clone(), 50));

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
    state: Rc<RefCell<SimulationState>>,
    selected: Rc<RefCell<NomVariant>>,
    amount: i32,
) -> impl FnMut() {
    let (noms, selected) = (state.borrow_mut().noms.clone(), selected.clone());
    move || {
        let mut noms_mut = noms.borrow_mut();
        for _ in 0..amount {
            let created_nom = Rc::new(RefCell::new(Nom::spawn_random(selected.borrow().clone())));
            if !state.borrow().devtools.random_spike_mutation
                && (*selected.borrow() == NomVariant::Default
                    || *selected.borrow() == NomVariant::GreenMutation
                    || *selected.borrow() == NomVariant::RedMutation
                    || *selected.borrow() == NomVariant::BlueMutation)
            {
                let amount = match state.borrow().devtools.spike_amount {
                    0 => 0,
                    1 => 1,
                    2 => 3,
                    3 => 6,
                    _ => panic!("Spawn random: incorrect slider value"),
                };
                created_nom.borrow_mut().spike_amount = amount;
            }
            if state.borrow().devtools.twin_mutation == NomTwins::Off {
                created_nom.borrow_mut().has_twin = false;
            } else if state.borrow().devtools.twin_mutation == NomTwins::On
                && (*selected.borrow() == NomVariant::Default
                    || *selected.borrow() == NomVariant::GreenMutation
                    || *selected.borrow() == NomVariant::RedMutation
                    || *selected.borrow() == NomVariant::BlueMutation)
            {
                created_nom.borrow_mut().has_twin = true;
            }
            noms_mut.push(created_nom);
        }
    }
}
