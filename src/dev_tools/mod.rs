use command_line::CommandLine;
use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use tab_selection::Tabs;
use tabs::nom_tools::NomTools;
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
    nom_tools: NomTools,
    current_tab: Tabs,
}

impl DevTools {
    pub fn new(state: Rc<RefCell<SimulationState>>) -> Self {
        let state = state.clone();
        Self {
            state: state.clone(),
            devtools_active: false,
            command_line: CommandLine::new(),
            simulation_tools: SimulationTools::new(state.clone()),
            nom_tools: NomTools::new(),
            current_tab: Tabs::Simulation,
        }
    }

    pub fn draw(&self) {
        if !self.devtools_active {
            return;
        };
        // Devtools side bar:
        draw_rectangle(0., 0., 400., screen_height(), Color::new(0., 0., 0., 0.8));
        self.draw_tab_selection();
        match self.current_tab {
            Tabs::Simulation => self.simulation_tools.draw(self.state.clone()),
            Tabs::Noms => self.nom_tools.draw(),
            _ => {}
        }

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
            self.update_tab_selection();
            match self.current_tab {
                Tabs::Simulation => self.simulation_tools.update(self.state.clone()),
                Tabs::Noms => self.nom_tools.update(),
                _ => {}
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.devtools_active
    }
}
