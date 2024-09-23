use crate::{generate_getters, generate_mut_getters};

pub struct Visuals {
    quadtree: bool,
    grid: bool,
    nom_wandering: bool,
    nom_orientation: bool,
    nom_target_orientation: bool,
    nom_detection_radius: bool,
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
        }
    }
}

generate_getters!(
  Visuals,
  quadtree: bool,
  grid: bool,
  nom_wandering: bool,
  nom_orientation: bool,
  nom_target_orientation: bool,
  nom_detection_radius: bool
);

generate_mut_getters!(
  Visuals,
  quadtree_mut: quadtree: bool,
  grid_mut: grid: bool,
  nom_wandering_mut: nom_wandering: bool,
  nom_orientation_mut: nom_orientation: bool,
  nom_target_orientation_mut: nom_target_orientation: bool,
  nom_detection_radius_mut: nom_detection_radius: bool
);
