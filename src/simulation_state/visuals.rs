pub struct Visuals {
    pub quadtree: bool,
    pub grid: bool,
    pub nom_wandering: bool,
    pub nom_orientation: bool,
    pub nom_target_orientation: bool,
    pub nom_detection_radius: bool,
    pub collisions: bool,
}

impl Visuals {
    pub fn new() -> Self {
        Self {
            quadtree: false,
            grid: false,
            nom_wandering: false,
            nom_orientation: false,
            nom_target_orientation: false,
            nom_detection_radius: false,
            collisions: false,
        }
    }
}
