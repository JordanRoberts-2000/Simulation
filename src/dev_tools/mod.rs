use command_line::CommandLine;
use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use toggles::Toggles;

use crate::nom::Nom;
use crate::utils::toggle::ToggleSwitch;

mod command_line;
mod toggles;

pub struct DevTools {
    devtools_active: bool,
    command_line: CommandLine,
    testing_visuals: bool,
    toggles: Toggles,
    noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
}

impl DevTools {
    pub fn new(noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>) -> Self {
        Self {
            devtools_active: false,
            command_line: CommandLine::new(),
            testing_visuals: false,
            toggles: Toggles::new(),
            noms,
        }
    }

    pub fn draw(&self) {
        if !self.devtools_active {
            return;
        };
        // draw all the toggles
        draw_rectangle(0., 0., 400., screen_height(), Color::new(0., 0., 0., 0.7));
        draw_line(400., 0., 400., screen_height(), 1.0, GRAY);
        draw_text("Quadgrid visuals", 20.0, 30.0, 24.0, WHITE);
        draw_text("Nom visuals", 20.0, 70.0, 24.0, WHITE);
        self.toggles.draw();
        self.command_line.draw();
    }

    pub fn is_active(&self) -> bool {
        self.devtools_active
    }

    pub fn handle_inputs(&mut self) {
        if is_key_pressed(KeyCode::LeftShift) {
            self.devtools_active = !self.devtools_active;
            if self.devtools_active {
                self.command_line.clear_input();
                // Flush any remaining input from before activation
                while let Some(_) = get_char_pressed() {}
            }
        }

        if self.devtools_active {
            self.command_line.handle_inputs(self.noms.clone());
            self.toggles.update();
        }
    }
}
