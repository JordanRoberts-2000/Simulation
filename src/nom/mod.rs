use ::rand::{thread_rng, Rng};
use draw::NomColors;
use macroquad::prelude::*;
use noise::Perlin;

mod collisions;
mod draw;
mod steering_behaviors;
mod update;
mod utils;

const NOM_SPAWN_DETECTION_RADIUS: f32 = 200.;
const NOM_SPAWN_SPEED: f32 = 50.;

pub struct NomStats {
    pub variant: NomVariant,
    pub age: f64,
    pub size: f32,
    pub max_size: f32,
    pub hunger: u32,
    pub state: NomState,
}

#[derive(Debug)]
pub enum NomState {
    Wandering,
}

#[derive(Clone, PartialEq, Debug)]
pub enum NomVariant {
    Default,
    BlueMutation,
    GreenMutation,
    RedMutation,
    Hedgehog,
    Shark,
    Whale,
    Death,
    Wendigo,
    Leviathan,
}

#[derive(Clone)]
pub struct Nom {
    colors: NomColors,
    pub size: f32,
    max_size: f32,
    age: f64,
    pub position: Vec2,
    pub variant: NomVariant,
    orientation: f32,
    target_position: Vec2,
    target_orientation: f32,
    current_speed: f32,
    max_speed: f32,
    acceleration: f32,
    turning_speed: f32,
    panicking: bool,
    spike_amount: u32,
    has_twin: bool,
    player_controlled: bool,
    mutation_variant: u32,
    perlin: Perlin,
    time_offset: f64,
    look_ahead_distance: f32,
    look_ahead_size: f32,
    look_ahead_target: Vec2,
    stats_active: bool,
}

impl Nom {
    pub fn new(position: Vec2, variant: NomVariant) -> Self {
        let starting_orientation: f32 = thread_rng().gen_range(0.0..=359.0);
        let size = Nom::get_max_size(variant.clone());
        Self {
            colors: Nom::get_colors(&variant),
            size,
            max_size: Nom::get_max_size(variant.clone()),
            age: get_time(),
            variant,
            position,
            target_position: Vec2::ZERO,
            current_speed: 0.0,                             // Scalar speed
            orientation: starting_orientation.to_radians(), // Initial orientation
            target_orientation: starting_orientation.to_radians(),
            max_speed: NOM_SPAWN_SPEED,
            acceleration: NOM_SPAWN_SPEED / 2.0,
            turning_speed: (120.0 as f32).to_radians(),
            panicking: false,
            has_twin: false,
            spike_amount: 0,
            player_controlled: false,
            mutation_variant: thread_rng().gen_range(0..=2),
            perlin: Perlin::new(1),
            time_offset: rand::gen_range(0.0, 1000.0),
            look_ahead_distance: size * 2.0,
            look_ahead_size: size * 2.0,
            look_ahead_target: vec2(0.0, 0.0),
            stats_active: false,
        }
    }

    pub fn spawn_random(variant: NomVariant) -> Self {
        // get random position,
        // check quadtree for collisions
        // add to quadtree
        let size = Nom::get_max_size(variant.clone());
        let starting_orientation: f32 = thread_rng().gen_range(0.0..=359.0);
        let mut rng = thread_rng();
        let mutation_variant = if variant == NomVariant::Hedgehog {
            thread_rng().gen_range(1..=2)
        } else {
            thread_rng().gen_range(0..=2)
        };
        let has_twin = match variant {
            NomVariant::Default
            | NomVariant::BlueMutation
            | NomVariant::RedMutation
            | NomVariant::GreenMutation => {
                if thread_rng().gen_range(0..100) < 15 {
                    true
                } else {
                    false
                }
            }
            _ => false,
        };
        let spike_amount: u32 = match variant {
            NomVariant::GreenMutation => {
                if thread_rng().gen_range(0..100) < 70 {
                    let number_option = vec![1, 3, 6];
                    let random_index = thread_rng().gen_range(0..3);
                    number_option[random_index]
                } else {
                    0
                }
            }
            NomVariant::BlueMutation | NomVariant::Default | NomVariant::RedMutation => {
                if thread_rng().gen_range(0..100) < 4 {
                    let number_option = vec![1, 3, 6];
                    let random_index = thread_rng().gen_range(0..3);
                    number_option[random_index]
                } else {
                    0
                }
            }
            _ => 0,
        };
        Self {
            colors: Nom::get_colors(&variant),
            size,
            max_size: size,
            age: get_time(),
            position: vec2(
                rng.gen_range(size / 2.0..screen_width() - size / 2.0),
                rng.gen_range(size / 2.0..screen_height() - size / 2.0),
            ),
            variant,
            target_position: Vec2::ZERO,
            current_speed: 0.0,                             // Scalar speed
            orientation: starting_orientation.to_radians(), // Initial orientation
            target_orientation: starting_orientation.to_radians(),
            max_speed: NOM_SPAWN_SPEED,
            acceleration: NOM_SPAWN_SPEED / 2.0,
            turning_speed: (120.0 as f32).to_radians(),
            panicking: false,
            has_twin,
            spike_amount,
            player_controlled: false,
            mutation_variant,
            perlin: Perlin::new(1),
            time_offset: rand::gen_range(0.0, 1000.0),
            look_ahead_distance: size * 2.0,
            look_ahead_size: size * 2.0,
            look_ahead_target: vec2(0.0, 0.0),
            stats_active: false,
        }
    }

    pub fn display_new(position: Vec2, variant: NomVariant) -> Self {
        let size = match variant {
            NomVariant::Leviathan => 80.0,
            NomVariant::Whale => 50.0,
            NomVariant::Hedgehog => 18.0,
            NomVariant::Wendigo => 32.0,
            NomVariant::Shark => 32.0,
            _ => 24.0,
        };
        let mutation_variant = if variant == NomVariant::Hedgehog {
            thread_rng().gen_range(1..=2)
        } else {
            thread_rng().gen_range(0..=2)
        };
        Self {
            colors: Nom::get_colors(&variant),
            size,
            max_size: size,
            age: get_time(),
            variant,
            position,
            target_position: Vec2::ZERO,
            current_speed: 0.0, // Scalar speed
            orientation: 0.0,   // Initial orientation
            target_orientation: 0.0,
            max_speed: NOM_SPAWN_SPEED,
            acceleration: NOM_SPAWN_SPEED / 2.0,
            turning_speed: (120.0 as f32).to_radians(),
            panicking: false,
            has_twin: false,
            spike_amount: 0,
            player_controlled: false,
            mutation_variant,
            perlin: Perlin::new(1),
            time_offset: rand::gen_range(0.0, 1000.0),
            look_ahead_distance: size * 1.25,
            look_ahead_size: size,
            look_ahead_target: vec2(0.0, 0.0),
            stats_active: false,
        }
    }

    fn get_max_size(variant: NomVariant) -> f32 {
        match variant {
            NomVariant::Hedgehog => 18.0,
            NomVariant::Wendigo => 32.0,
            NomVariant::Whale => 160.0,
            NomVariant::Leviathan => 400.0,
            NomVariant::Shark => 32.0,
            _ => 32.0,
        }
    }

    pub fn get_stats(&self) -> NomStats {
        return NomStats {
            variant: self.variant.clone(),
            age: self.age,
            size: self.size,
            max_size: self.max_size,
            hunger: 200,
            state: NomState::Wandering,
        };
    }

    pub fn show_stats(&mut self) {
        self.stats_active = true;
    }

    pub fn hide_stats(&mut self) {
        self.stats_active = false;
    }

    pub fn set_variant(&mut self, variant: NomVariant) {
        self.colors = Nom::get_colors(&variant);
        self.variant = variant;
    }

    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }
}
