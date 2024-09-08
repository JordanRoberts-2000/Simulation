use command_line::CommandLine;
use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nom::Nom;
use crate::quadtree::Quadtree;
use toggles::Toggles;

mod command_line;
mod toggles;

pub struct DevTools {
    devtools_active: bool,
    command_line: CommandLine,
    pub nom_visuals_active: Rc<RefCell<bool>>,
    pub quadtree_visuals_active: Rc<RefCell<bool>>,
    toggles: Toggles,
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
            nom_visuals_active: nom_visuals_active.clone(),
            quadtree_visuals_active: quadtree_visuals_active.clone(),
            toggles: Toggles::new(),
            noms,
            quadtree: quadtree.clone(),
        };

        devtools.toggles.add_toggle(
            vec2(350., 75.),
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
            self.command_line
                .handle_inputs(self.noms.clone(), self.quadtree.clone());
            self.toggles.update();
        }
    }
}
