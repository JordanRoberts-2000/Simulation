use std::{cell::RefCell, rc::Rc};

use behaviour::Behaviour;
use visuals::Visuals;

use crate::{generate_getters, generate_mut_getters, nom::Nom, quadtree::Quadtree};

mod behaviour;
mod visuals;

pub struct SimulationState {
    noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
    quadtree: Rc<RefCell<Quadtree>>,
    visuals: Visuals,
    behaviour: Behaviour,
}

impl SimulationState {
    pub fn new(noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>, quadtree: Rc<RefCell<Quadtree>>) -> Self {
        Self {
            noms,
            quadtree,
            visuals: Visuals::new(),
            behaviour: Behaviour::new(),
        }
    }

    pub fn visuals(&self) -> &Visuals {
        &self.visuals
    }

    pub fn visuals_mut(&mut self) -> &mut Visuals {
        &mut self.visuals
    }

    pub fn behaviors(&self) -> &Behaviour {
        &self.behaviour
    }

    pub fn behaviors_mut(&mut self) -> &mut Behaviour {
        &mut self.behaviour
    }
}

generate_getters!(
  SimulationState,
  noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
  quadtree: Rc<RefCell<Quadtree>>
);

generate_mut_getters!(
    SimulationState,
    // nom_mut: noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>, <-- not needed cause clone
    // quadtree_mut: quadtree: Rc<RefCell<Quadtree>>  <-- not needed
);
