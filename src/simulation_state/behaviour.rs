use crate::{generate_getters, generate_mut_getters};

pub struct Behaviour {
    movement: bool,
}

impl Behaviour {
    pub fn new() -> Self {
        Self { movement: true }
    }
}

generate_getters!(
  Behaviour,
  movement: bool
);

generate_mut_getters!(
  Behaviour,
  movement_mut: movement: bool
);
