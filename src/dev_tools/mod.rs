use command_line::CommandLine;
use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use tabs::simulation_tools::SimulationTools;

use crate::simulation_state::SimulationState;

mod command_line;
mod tab_selection;
mod tabs;

pub struct DevTools {
    state: Rc<RefCell<SimulationState>>,
    devtools_active: bool,
    command_line: CommandLine,
    simulation_tools: SimulationTools,
}

impl DevTools {
    pub fn new(state: Rc<RefCell<SimulationState>>) -> Self {
        let state = state.clone();
        Self {
            state: state.clone(),
            devtools_active: false,
            command_line: CommandLine::new(),
            simulation_tools: SimulationTools::new(state.clone()),
        }
    }

    pub fn draw(&self) {
        if !self.devtools_active {
            return;
        };
        // Devtools side bar:
        draw_rectangle(0., 0., 400., screen_height(), Color::new(0., 0., 0., 0.7));
        self.draw_tab_selection();
        self.simulation_tools.draw();

        draw_line(400., 0., 400., screen_height(), 1.0, GRAY);
        self.command_line.draw();
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::LeftShift) {
            self.devtools_active = !self.devtools_active;
            if self.devtools_active {
                self.command_line.clear_input();
                // Flush any remaining input from before activation
                while let Some(_) = get_char_pressed() {}
            }
        }
        if self.devtools_active {
            self.command_line.update(self.state.clone());
            self.simulation_tools.update(self.state.clone());
        }
    }

    pub fn is_active(&self) -> bool {
        self.devtools_active
    }
}
