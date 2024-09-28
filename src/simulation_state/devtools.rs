use crate::dev_tools::tabs::simulation_tools::nom_spawner::spawn_settings::{
    NomLifeCycle, NomTwins,
};

use super::visuals::Visuals;

pub struct DevtoolState {
    pub apply_to_all: bool,
    pub disable_movement: bool,
    pub random_spike_mutation: bool,
    pub spike_amount: u32,
    pub twin_mutation: NomTwins,
    pub nom_life_cycle: NomLifeCycle,
    pub visuals: Visuals,
}

impl DevtoolState {
    pub fn new() -> Self {
        Self {
            apply_to_all: false,
            disable_movement: false,
            random_spike_mutation: true,
            spike_amount: 0,
            twin_mutation: NomTwins::Random,
            nom_life_cycle: NomLifeCycle::Adult,
            visuals: Visuals::new(),
        }
    }
}
