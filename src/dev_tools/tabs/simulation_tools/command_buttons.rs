use std::cell::RefCell;
use std::rc::Rc;

use macroquad::prelude::*;

use crate::dev_tools::command_line::commands::handle_clear;
use crate::simulation_state::SimulationState;
use crate::utils::ui::button::Button;

use crate::dev_tools::SimulationTools;

impl SimulationTools {
    pub fn create_command_buttons(state: Rc<RefCell<SimulationState>>) -> Vec<Button> {
        let command_buttons_y: f32 = 668.0;

        let mut clear_button = Button::new("clear");
        clear_button.pos(28.0, command_buttons_y);
        clear_button.on_click(move || clear_noms(state.clone()));

        let mut restart_button = Button::new("restart");
        restart_button.pos(92.0, command_buttons_y);
        restart_button.on_click(restart_sim);

        vec![clear_button, restart_button]
    }

    pub fn draw_command_buttons(&self) {
        for button in self.command_buttons.iter() {
            button.draw();
        }
    }

    pub fn update_command_buttons(&mut self) {
        for button in self.command_buttons.iter_mut() {
            button.update();
        }
    }
}

fn clear_noms(state: Rc<RefCell<SimulationState>>) {
    let noms = state.borrow().noms().clone();
    handle_clear(noms.clone());
}

fn restart_sim() {
    println!("restart");
}
