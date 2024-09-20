use command_line::CommandLine;
use macroquad::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::nom::Nom;
use crate::quadtree::Quadtree;
use crate::utils::button::Button;
use crate::utils::draw::draw_rounded_rectangle;
use crate::utils::slider::Slider;
use crate::utils::toggle::ToggleSwitch;
use buttons::ButtonKeys;
use nom_spawner::NomSpawner;
use toggles::ToggleKeys;

mod buttons;
mod command_line;
mod nom_spawner;
mod toggles;

pub struct DevTools {
    devtools_active: bool,
    command_line: CommandLine,
    nom_spawner: NomSpawner,
    pub nom_visuals_active: Rc<RefCell<bool>>,
    pub quadtree_visuals_active: Rc<RefCell<bool>>,
    toggles: HashMap<ToggleKeys, ToggleSwitch>,
    buttons: HashMap<ButtonKeys, Button>,
    nom_variant_selected_index: (usize, usize),
    noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
    quadtree: Rc<RefCell<Quadtree>>,
}

impl DevTools {
    pub fn new(noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>, quadtree: Rc<RefCell<Quadtree>>) -> Self {
        let nom_visuals_active = Rc::new(RefCell::new(false));
        let quadtree_visuals_active = Rc::new(RefCell::new(false));
        let mut devtools = DevTools {
            devtools_active: false,
            command_line: CommandLine::new(),
            nom_spawner: NomSpawner::new(),
            nom_visuals_active: nom_visuals_active.clone(),
            quadtree_visuals_active: quadtree_visuals_active.clone(),
            toggles: DevTools::create_toggles(
                nom_visuals_active.clone(),
                quadtree_visuals_active.clone(),
            ),
            buttons: DevTools::create_buttons(),
            nom_variant_selected_index: (0, 0),
            noms,
            quadtree: quadtree.clone(),
        };

        let command_buttons_y: f32 = 659.0;
        devtools.set_button(ButtonKeys::Clear, 28.0, command_buttons_y, "Clear");
        devtools.set_button(ButtonKeys::Freeze, 100.0, command_buttons_y, "Freeze");
        devtools.set_button(ButtonKeys::Restart, 179.0, command_buttons_y, "Restart");

        let spawn_buttons_y: f32 = 620.0;
        devtools.set_button(ButtonKeys::SpawnOne, 20.0, spawn_buttons_y, "Spawn Nom");
        devtools.set_button(ButtonKeys::SpawnFive, 184.0, spawn_buttons_y, "+5");
        devtools.set_button(ButtonKeys::SpawnTen, 238.0, spawn_buttons_y, "+10");
        devtools.set_button(ButtonKeys::SpawnTwenty, 288.0, spawn_buttons_y, "+20");
        devtools.set_button(ButtonKeys::SpawnFifty, 338.0, spawn_buttons_y, "+50");

        devtools.set_toggle(ToggleKeys::QuadTree, 350.0, 35.0);
        devtools.set_toggle(ToggleKeys::NomVisuals, 350.0, 115.0);
        devtools.set_toggle(ToggleKeys::GRID, 350.0, 75.0);

        devtools
    }

    pub fn draw(&self) {
        if !self.devtools_active {
            return;
        };
        // Devtools side bar:
        draw_rectangle(0., 0., 400., screen_height(), Color::new(0., 0., 0., 0.7));
        // Toggles:
        draw_text("Quadgrid visuals", 20.0, 40.0, 24.0, WHITE);
        draw_text("Spatial grid visuals", 20.0, 80.0, 24.0, WHITE);
        draw_text("Nom visuals", 20.0, 120.0, 24.0, WHITE);
        // Nom spawner:
        self.nom_spawner.draw();

        draw_line(400., 0., 400., screen_height(), 1.0, GRAY);
        for (_, button) in self.buttons.iter() {
            button.draw();
        }
        for (_, toggle) in self.toggles.iter() {
            toggle.draw();
        }
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
            self.command_line
                .handle_inputs(self.noms.clone(), self.quadtree.clone());
            self.nom_spawner.update();
            for (_, button) in self.buttons.iter_mut() {
                button.update();
            }
            for (_, toggle) in self.toggles.iter_mut() {
                toggle.update();
            }
        }
    }
}
