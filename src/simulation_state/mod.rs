use std::{cell::RefCell, rc::Rc};

use behaviour::Behaviour;
use visuals::Visuals;

use crate::{nom::Nom, quadtree::Quadtree};

mod behaviour;
mod visuals;

pub struct SimulationState {
    pub noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
    pub quadtree: Rc<RefCell<Quadtree>>,
    pub apply_to_all: bool,
    pub visuals: Visuals,
    pub behaviour: Behaviour,
}

impl SimulationState {
    pub fn new(noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>, quadtree: Rc<RefCell<Quadtree>>) -> Self {
        Self {
            noms,
            quadtree,
            apply_to_all: false,
            visuals: Visuals::new(),
            behaviour: Behaviour::new(),
        }
    }
}

// generate_getters!(
//   SimulationState,
//   noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
//   quadtree: Rc<RefCell<Quadtree>>
// );
