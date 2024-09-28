pub struct Behaviour {
    pub disable_movement: bool,
    pub spike_mutation: bool,
    pub twin_mutation: bool,
}

impl Behaviour {
    pub fn new() -> Self {
        Self {
            disable_movement: false,
            spike_mutation: false,
            twin_mutation: false,
        }
    }
}
