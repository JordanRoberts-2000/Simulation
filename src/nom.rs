use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

// use crate::plants::Plant;

const NOM_SPAWN_DETECTION_RADIUS: f32 = 200.;

pub struct Stats {
    pub current_speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub orientation: f32,
}
// WHALE
// const NOM_SPAWN_SIZE: f32 = 70.;
// const NOM_BORDER_THICKNESS: f32 = 1.5;
// const NOM_SPAWN_SPEED: f32 = 10.;
// const NOM_COLOR: Color = Color::new(0.5764706, 0.2, 0.91764706, 1.0);
// const NOM_BORDER_COLOR: Color = Color::new(0.9764706, 0.2, 0.91764706, 0.5);

// DEFAULT
const NOM_SPAWN_SIZE: f32 = 16.;
const NOM_BORDER_THICKNESS: f32 = 2.0;
const NOM_SPAWN_SPEED: f32 = 50.;
const NOM_SPAWN_SPRINT_SPEED: f32 = 200.;
// const NOM_BORDER_COLOR: Color = Color::new(0.9961, 0.0, 0.9961, 1.0);
// const NOM_COLOR: Color = Color::new(0.2, 0.0, 0.2, 1.0);
const NOM_COLOR: Color = Color::new(0.2, 0.0, 0.2, 1.0);
const NOM_BORDER_COLOR: Color = Color::new(0.6961, 0.0, 0.9961, 1.0);

// Hedgehogs ??
// const NOM_SPAWN_SIZE: f32 = 10.;
// const NOM_BORDER_THICKNESS: f32 = 1.0;
// const NOM_SPAWN_SPEED: f32 = 200.;
// const NOM_BORDER_COLOR: Color = Color::new(0.99607843, 0.8039216, 0.827451, 1.0);
// const NOM_COLOR: Color = Color::new(0.3137255, 0.02745098, 0.14117648, 1.0);
// const NOM_COLOR: Color = Color::new(0.88235295, 0.11372549, 0.28235295, 1.0);

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
}

impl Nom {
    pub fn new(position: Vec2, variant: NomVariant) -> Self {
        Self {
            size: Nom::get_max_size(variant.clone()),
            max_size: Nom::get_max_size(variant.clone()),
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
        }
    }

    pub fn update(&mut self) {
        let delta_time = get_frame_time();
        if self.player_controlled {
            let (mouse_x, mouse_y) = mouse_position();
            self.target_position = Vec2::new(mouse_x, mouse_y);
            if is_key_down(KeyCode::Space) {
                self.panicking = true;
            }
            if is_key_released(KeyCode::Space) {
                self.panicking = false;
            }
        }
        if self.player_controlled {
            self.update_orientation(&delta_time);
            self.update_position(&delta_time);
        }
    }

    pub fn spawn_random() -> Self {
        // get random position,
        // check quadtree for collisions
        // add to quadtree
        let mut rng = thread_rng();
        let size: f32 = 24.;
        Self {
            size,
            max_size: 24.0,
            age: get_time(),
            position: vec2(
                rng.gen_range(size / 2.0..screen_width() / 2.0 - size / 2.0),
                rng.gen_range(size / 2.0..screen_height() / 2.0 - size / 2.0),
            ),
            variant: NomVariant::Default,
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
        }
    }

    pub fn draw(&self, testing_visuals: Rc<RefCell<bool>>) {
        self.draw_body();
        self.draw_mutation();

        // Draw the direction the object is facing (orientation line)
        if *testing_visuals.borrow() {
            let line_length = 30.0;
            let x2 = self.position.x + self.orientation.to_radians().cos() * line_length;
            let y2 = self.position.y + self.orientation.to_radians().sin() * line_length;
            draw_line(self.position.x, self.position.y, x2, y2, 2.0, RED);
        }
    }

    pub fn draw_body(&self) {
        let (color, border_color) = match self.variant {
            NomVariant::Wendigo => (Color::from_hex(0x450a0a), Color::from_hex(0xdc2626)),
            NomVariant::Death => (BLACK, Color::from_hex(0x525252)),
            NomVariant::Leviathan => (
                Color::new(0.2, 0.0, 0.2, 1.0),
                Color::new(0.2, 0.0, 0.2, 1.0),
            ),
            NomVariant::Whale => (
                Color::new(0.2, 0.0, 0.2, 1.0),
                Color::new(0.2, 0.0, 0.2, 1.0),
            ),
            _ => (
                Color::new(0.2, 0.0, 0.2, 1.0),
                Color::new(0.9961, 0.0, 0.9961, 1.0),
            ),
        };

        draw_circle(
            self.position[0],
            self.position[1],
            self.size / 2.0,
            if self.size >= 4.0 {
                border_color
            } else {
                color
            },
        );
        match self.variant {
            NomVariant::Whale => {
                for i in 0..10 {
                    let size = self.size / 20.0;
                    let radius = size + (i as f32 * size);
                    let alpha = 0.05 * i as f32; // Decrease alpha for outer circles
                    let color = Color::new(0.2764706, 0.2, 0.91764706, alpha); // Semi-transparent color

                    draw_circle(self.position[0], self.position[1], radius, color);
                }
            }
            NomVariant::Leviathan => {
                for i in 0..20 {
                    let size = self.size / 20.0;
                    let radius = size + i as f32 * size;
                    let alpha = 0.05 * (10 - i) as f32; // Decrease alpha for outer circles
                    let color = Color::new(0.9764706, 0.2, 0.21764706, alpha); // Semi-transparent color

                    draw_circle(self.position[0], self.position[1], radius, color);
                }
            }
            _ => {
                if self.size >= 4.0 {
                    draw_circle(
                        self.position[0],
                        self.position[1],
                        (self.size / 2.0) - if self.size >= 14.0 { 1.5 } else { 1.0 },
                        color,
                    );
                }
            }
        }
    }

    pub fn draw_mutation(&self) {
        if self.variant == NomVariant::Leviathan || self.variant == NomVariant::Whale {
            return;
        }
        let mutation_color = match self.variant {
            NomVariant::Default => Color::new(0.9961, 0.0, 0.9961, 1.0),
            NomVariant::BlueMutation => Color::new(0.231, 0.510, 0.965, 1.0),
            NomVariant::GreenMutation => GREEN,
            NomVariant::RedMutation => RED,
            NomVariant::Hedgehog => YELLOW,
            NomVariant::Wendigo => RED,
            NomVariant::Death => Color::from_hex(0x991b1b),
            _ => Color::new(0.9961, 0.0, 0.9961, 1.0),
        };
        if self.mutation_variant == 0 {
            self.draw_mutation_bubble(vec2(5.0, -1.0), false, mutation_color);
            self.draw_mutation_bubble(vec2(-2.0, -3.0), true, mutation_color);
            self.draw_mutation_bubble(vec2(-3.0, 2.0), false, mutation_color);
            self.draw_mutation_bubble(vec2(3.0, 4.0), false, mutation_color);
        }
        if self.mutation_variant == 1 {
            self.draw_mutation_bubble(vec2(0.0, -3.0), true, mutation_color);
            self.draw_mutation_bubble(vec2(-3.0, 2.0), false, mutation_color);
            self.draw_mutation_bubble(vec2(3.0, 1.0), false, mutation_color);
        }
        if self.mutation_variant == 2 {
            self.draw_mutation_bubble(vec2(1.0, -2.0), true, mutation_color);
            self.draw_mutation_bubble(vec2(-2.0, 3.0), false, mutation_color);
        }
    }

    pub fn draw_mutation_bubble(&self, position: Vec2, large: bool, mutation_color: Color) {
        let start_size: f32 = 8.0;
        let finish_size: f32 = 18.0;
        draw_circle(
            self.position[0]
                + (((self.size.clamp(start_size, finish_size) - start_size)
                    / (finish_size - start_size))
                    * position.x)
                    .round(),
            self.position[1]
                + (((self.size.clamp(start_size, finish_size) - start_size)
                    / (finish_size - start_size))
                    * position.y)
                    .round(),
            if large && self.size >= 18.0 { 2.0 } else { 1.0 },
            mutation_color,
        );
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
        }
    }

    pub fn update_position(&mut self, delta_time: &f32) {
        self.current_speed =
            (self.current_speed + self.acceleration * delta_time).min(self.max_speed);
        self.position.x += self.current_speed * self.orientation.to_radians().cos() * delta_time;
        self.position.y += self.current_speed * self.orientation.to_radians().sin() * delta_time;
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

    pub fn check_collision(&self, nom: &Nom) -> bool {
        // if !self.bounding_box_collision(nom) {
        //     return false;
        // }
        return self.circle_collision(nom);
    }

    fn bounding_box_collision(&self, nom: &Nom) -> bool {
        !(self.position.x + self.size / 2.0 < nom.position.x - nom.size / 2.0 || // Right of self.position is left of nom.position
        self.position.x - self.size / 2.0 > nom.position.x + nom.size / 2.0 || // Left of self.position is right of nom.position
        self.position.y + self.size / 2.0 < nom.position.y - nom.size / 2.0 || // Bottom of self.position is above nom.position
        self.position.y - self.size / 2.0 > nom.position.y + nom.size / 2.0) // Top of circle1 is below circle2
    }

    fn circle_collision(&self, nom: &Nom) -> bool {
        let dx = nom.position.x - self.position.x;
        let dy = nom.position.y - self.position.y;
        let distance_squared = dx * dx + dy * dy;
        let radii_sum = self.size / 2.0 + nom.size / 2.0;
        return distance_squared <= radii_sum * radii_sum;
    }

    pub fn update_orientation(&mut self, delta_time: &f32) {
        let dx = self.target_position.x - self.position.x;
        let dy = self.target_position.y - self.position.y;
        let target_angle = dy.atan2(dx).to_degrees();
        self.target_orientation = (target_angle + 360.0) % 360.0;

        // Normalize the current orientation to be between 0 and 360 degrees
        self.orientation = (self.orientation + 360.0) % 360.0;

        // Calculate the angle difference and determine the shortest rotation direction
        let mut angle_difference = self.target_orientation - self.orientation;
        if angle_difference > 180.0 {
            angle_difference -= 360.0;
        } else if angle_difference < -180.0 {
            angle_difference += 360.0;
        }

        // Apply turning (rotate towards the target orientation smoothly)
        let turn = self.turning_speed * delta_time;
        if angle_difference.abs() > turn {
            self.orientation += turn * angle_difference.signum();
        } else {
            self.orientation = self.target_orientation;
        }

        // Ensure orientation is in the [0, 360) range
        self.orientation = (self.orientation + 360.0) % 360.0;
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
