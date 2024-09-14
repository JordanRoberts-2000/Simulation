use command_line::CommandLine;
use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nom::Nom;
use crate::quadtree::Quadtree;
use crate::utils::draw::draw_rounded_rectangle;
use nom_spawner::NomSpawner;
use toggles::Toggles;

mod command_line;
mod nom_spawner;
mod toggles;

pub struct DevTools {
    devtools_active: bool,
    command_line: CommandLine,
    nom_spawner: NomSpawner,
    pub nom_visuals_active: Rc<RefCell<bool>>,
    pub quadtree_visuals_active: Rc<RefCell<bool>>,
    toggles: Toggles,
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
            toggles: Toggles::new(),
            nom_variant_selected_index: (0, 0),
            noms,
            quadtree: quadtree.clone(),
        };

        devtools.toggles.add_toggle(
            vec2(350., 115.),
            Box::new({
                let nom_visuals_active = nom_visuals_active.clone();
                move || {
                    *nom_visuals_active.borrow_mut() = true;
                }
            }),
            Box::new({
                let nom_visuals_active = nom_visuals_active.clone();
                move || {
                    *nom_visuals_active.borrow_mut() = false;
                }
            }),
        );
        devtools.toggles.add_toggle(
            vec2(350., 35.),
            Box::new({
                let quadtree_visuals_active = quadtree_visuals_active.clone();
                move || {
                    *quadtree_visuals_active.borrow_mut() = true;
                }
            }),
            Box::new({
                let quadtree_visuals_active = quadtree_visuals_active.clone();
                move || {
                    *quadtree_visuals_active.borrow_mut() = false;
                }
            }),
        );

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
        draw_rounded_rectangle(20.0, 570.0, 160.0, 30.0, 5.0, DARKGRAY);
        draw_rounded_rectangle(200.0, 570.0, 30.0, 30.0, 5.0, DARKGRAY);
        draw_rounded_rectangle(250.0, 570.0, 30.0, 30.0, 5.0, DARKGRAY);
        draw_rounded_rectangle(300.0, 570.0, 30.0, 30.0, 5.0, DARKGRAY);
        draw_rounded_rectangle(350.0, 570.0, 30.0, 30.0, 5.0, DARKGRAY);
        draw_line(400., 0., 400., screen_height(), 1.0, GRAY);
        draw_text("Twins", 20.0, 510.0, 20.0, WHITE);
        draw_text("Spikes", 20.0, 550.0, 20.0, WHITE);
        draw_text(
            "Stage, egg, baby, adult, old, dead, zombie",
            20.0,
            530.0,
            20.0,
            WHITE,
        );
        draw_text("Random", 260.0, 550.0, 16.0, WHITE);
        draw_rounded_rectangle(100.0, 540.0, 140.0, 4.0, 2.0, WHITE);
        draw_rounded_rectangle(20.0, 620.0, 80.0, 30.0, 5.0, DARKGRAY);
        draw_rounded_rectangle(20.0 + 1.0, 620.0 + 1.0, 80.0 - 2.0, 30.0 - 2.0, 5.0, BLACK);
        draw_text("Clear all", 28.0, 639.0, 16.0, WHITE);
        draw_text("Spawn plants", 120.0, 639.0, 16.0, WHITE);
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
            self.command_line
                .handle_inputs(self.noms.clone(), self.quadtree.clone());
            self.toggles.handle_inputs();
            self.nom_spawner.handle_inputs();
        }
    }
}
