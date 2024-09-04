use macroquad::prelude::*;

use super::plant::{Plant, PlantSize};

pub enum ClusterSize {
    Xs,
    Sm,
    Md,
}

pub struct Cluster {
    position: [f32; 2],
    plant_vec: Vec<Plant>,
}

impl Cluster {
    pub fn new(size: ClusterSize, position: [f32; 2]) -> Cluster {
        let cluster = match size {
            ClusterSize::Xs => {
                return Cluster::create_xs_cluster(position);
            }
            ClusterSize::Sm => {
                return Cluster::create_xs_cluster(position);
            }
            ClusterSize::Md => {
                return Cluster::create_xs_cluster(position);
            }
        };
        return cluster;
    }

    pub fn create_xs_cluster(position: [f32; 2]) -> Cluster {
        // randomly choose size
        return Cluster {
            position: [30.0, 50.0],
            plant_vec: vec![Plant::new([35.0, 45.0], PlantSize::Sm)],
        };
    }

    fn create_sm_cluster() -> Cluster {
        // randomly choose size and positions
        return Cluster {
            position: [30.0, 50.0],
            plant_vec: vec![Plant::new([35.0, 45.0], PlantSize::Sm)],
        };
    }
}

// clusters have variations
// random, for example:
// single/xs -> 4 different sizes xs / sm / md / lg
// double/sm -> sm + xs / medium + xs / lg + sm
