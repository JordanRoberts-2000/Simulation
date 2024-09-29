use std::{cell::RefCell, rc::Rc};

use devtools::DevtoolState;

use crate::{nom::Nom, quadtree::Quadtree};

mod devtools;
mod visuals;

pub struct SimulationState {
    pub noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
    pub quadtree: Rc<RefCell<Quadtree>>,
    pub devtools: DevtoolState,
    pub selected_nom: Rc<RefCell<Option<Rc<RefCell<Nom>>>>>,
}

impl SimulationState {
    pub fn new(noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>, quadtree: Rc<RefCell<Quadtree>>) -> Self {
        Self {
            noms,
            quadtree,
            devtools: DevtoolState::new(),
            selected_nom: Rc::new(RefCell::new(None)),
        }
    }
}

// generate_getters!(
//   SimulationState,
//   noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
//   quadtree: Rc<RefCell<Quadtree>>
// );
