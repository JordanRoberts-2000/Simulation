use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use noise::Perlin;

mod collisions;
mod draw;
mod steering_behaviors;
mod update;
mod utils;

const NOM_SPAWN_DETECTION_RADIUS: f32 = 200.;
const NOM_SPAWN_SPEED: f32 = 50.;

pub struct Stats {
    pub current_speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub orientation: f32,
}

#[derive(Clone, PartialEq)]
pub enum NomVariant {
    Default,
    BlueMutation,
    GreenMutation,
    RedMutation,
    Hedgehog,
    Whale,
    Death,
    Wendigo,
    Leviathan,
}

#[derive(Clone)]
pub struct Nom {
    pub size: f32,
    max_size: f32,
    age: f64,
    pub position: Vec2,
    variant: NomVariant,
    orientation: f32,
    target_position: Vec2,
    target_orientation: f32,
    current_speed: f32,
    max_speed: f32,
    acceleration: f32,
    turning_speed: f32,
    panicking: bool,
    player_controlled: bool,
    mutation_variant: u32,
    perlin: Perlin,
    time_offset: f64,
    look_ahead_distance: f32,
    look_ahead_size: f32,
    look_ahead_target: Vec2,
}

impl Nom {
    pub fn new(position: Vec2, variant: NomVariant) -> Self {
        let starting_orientation: f32 = thread_rng().gen_range(0.0..=359.0);
        let size = Nom::get_max_size(variant.clone());
        Self {
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
            turning_speed: 180.0, // Degrees per second
            panicking: false,
            player_controlled: false,
            mutation_variant: thread_rng().gen_range(0..=2),
            perlin: Perlin::new(1),
            time_offset: rand::gen_range(0.0, 1000.0),
            look_ahead_distance: size * 2.0,
            look_ahead_size: size * 2.0,
            look_ahead_target: vec2(0.0, 0.0),
        }
    }

    pub fn spawn_random() -> Self {
        // get random position,
        // check quadtree for collisions
        // add to quadtree
        let starting_orientation: f32 = thread_rng().gen_range(0.0..=359.0);
        let mut rng = thread_rng();
        let size: f32 = 24.;
        Self {
            size,
            max_size: 24.0,
            age: get_time(),
            position: vec2(
                rng.gen_range(size / 2.0..screen_width() - size / 2.0),
                rng.gen_range(size / 2.0..screen_height() - size / 2.0),
            ),
            variant: NomVariant::Default,
            target_position: Vec2::ZERO,
            current_speed: 0.0,                             // Scalar speed
            orientation: starting_orientation.to_radians(), // Initial orientation
            target_orientation: starting_orientation.to_radians(),
            max_speed: NOM_SPAWN_SPEED,
            acceleration: NOM_SPAWN_SPEED / 2.0,
            turning_speed: 180.0, // Degrees per second
            panicking: false,
            player_controlled: false,
            mutation_variant: thread_rng().gen_range(0..=2),
            perlin: Perlin::new(1),
            time_offset: rand::gen_range(0.0, 1000.0),
            look_ahead_distance: size * 1.25,
            look_ahead_size: size,
            look_ahead_target: vec2(0.0, 0.0),
        }
    }

    pub fn display_new(position: Vec2, variant: NomVariant) -> Self {
        let size = match variant {
            NomVariant::Leviathan => 80.0,
            NomVariant::Whale => 50.0,
            NomVariant::Hedgehog => 18.0,
            NomVariant::Wendigo => 32.0,
            _ => 24.0,
        };
        Self {
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
            turning_speed: 180.0, // Degrees per second
            panicking: false,
            player_controlled: false,
            mutation_variant: thread_rng().gen_range(0..=2),
            perlin: Perlin::new(1),
            time_offset: rand::gen_range(0.0, 1000.0),
            look_ahead_distance: size * 1.25,
            look_ahead_size: size,
            look_ahead_target: vec2(0.0, 0.0),
        }
    }

    fn get_max_size(variant: NomVariant) -> f32 {
        match variant {
            NomVariant::Hedgehog => 18.0,
            NomVariant::Wendigo => 32.0,
            NomVariant::Whale => 160.0,
            NomVariant::Leviathan => 400.0,
            _ => 24.0,
        }
    }

    pub fn get_stats(&self) -> Stats {
        return Stats {
            current_speed: self.current_speed,
            max_speed: self.max_speed,
            acceleration: self.acceleration,
            orientation: self.orientation,
        };
    }
}
