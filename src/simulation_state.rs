use std::{cell::RefCell, rc::Rc};

use crate::{nom::Nom, plants::Plants, quadtree::Quadtree};

pub struct SimulationState {
    noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
    quadtree: Rc<RefCell<Quadtree>>,
    plants: Plants,
}

impl SimulationState {
    pub fn new(
        noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
        quadtree: Rc<RefCell<Quadtree>>,
        plants: Plants,
    ) -> Self {
        Self {
            noms,
            quadtree,
            plants,
        }
    }

    pub fn get_noms(&self) -> Rc<RefCell<Vec<Rc<RefCell<Nom>>>>> {
        self.noms.clone()
    }

    pub fn get_quadtree(&self) -> Rc<RefCell<Quadtree>> {
        self.quadtree.clone()
    }
}
