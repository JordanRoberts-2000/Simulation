use crate::{generate_getters, generate_mut_getters};

pub struct Behaviour {
    movement: bool,
    spike_mutation: bool,
    twin_mutation: bool,
}

impl Behaviour {
    pub fn new() -> Self {
        Self {
            movement: true,
            spike_mutation: false,
            twin_mutation: false,
        }
    }
}

generate_getters!(
  Behaviour,
  movement: bool,
  spike_mutation: bool,
  twin_mutation: bool
);

generate_mut_getters!(
  Behaviour,
  movement_mut: movement: bool,
  spike_mutation_mut: spike_mutation: bool,
  twin_mutation_mut: twin_mutation: bool
);
