use macroquad::prelude::*;

use crate::utils::toggle::ToggleSwitch;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::DevTools;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ToggleKeys {
    QuadTree,
    GRID,
    NomVisuals,
}

impl DevTools {
    pub fn create_toggles(
        nom_visuals_active: Rc<RefCell<bool>>,
        quadtree_visuals_active: Rc<RefCell<bool>>,
    ) -> HashMap<ToggleKeys, ToggleSwitch> {
        let mut toggles = HashMap::new();
        toggles.insert(
            ToggleKeys::NomVisuals,
            ToggleSwitch::new(
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
            ),
        );
        toggles.insert(
            ToggleKeys::QuadTree,
            ToggleSwitch::new(
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
            ),
        );
        toggles.insert(
            ToggleKeys::GRID,
            ToggleSwitch::new(
                Box::new(|| {
                    println!("Grid toggle on");
                }),
                Box::new(|| {
                    println!("Grid toggle off");
                }),
            ),
        );
        toggles
    }

    pub fn set_toggle(&mut self, key: ToggleKeys, x: f32, y: f32) {
        if let Some(toggle) = self.toggles.get_mut(&key) {
            toggle.set(x, y);
        } else {
            panic!("Toggle with key {:?} not found", key);
        }
    }
}
