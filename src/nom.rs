use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

// use crate::plants::Plant;

const NOM_SPAWN_DETECTION_RADIUS: f32 = 200.;

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
const NOM_BORDER_COLOR: Color = Color::new(0.9961, 0.0, 0.9961, 1.0);
const NOM_COLOR: Color = Color::new(0.2, 0.0, 0.2, 1.0);

// Hedgehogs ??
// const NOM_SPAWN_SIZE: f32 = 10.;
// const NOM_BORDER_THICKNESS: f32 = 1.0;
// const NOM_SPAWN_SPEED: f32 = 200.;
// const NOM_BORDER_COLOR: Color = Color::new(0.99607843, 0.8039216, 0.827451, 1.0);
// const NOM_COLOR: Color = Color::new(0.3137255, 0.02745098, 0.14117648, 1.0);
// const NOM_COLOR: Color = Color::new(0.88235295, 0.11372549, 0.28235295, 1.0);

pub struct Stats {
    pub current_speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub orientation: f32,
}

#[derive(Clone)]
pub struct Nom {
    pub size: f32,
    pub position: Vec2,
    orientation: f32,
    target_position: Vec2,
    target_orientation: f32,
    current_speed: f32,
    max_speed: f32,
    acceleration: f32,
    turning_speed: f32,
    panicking: bool,
    pub temp_is_colliding: bool,
    player_controlled: bool,
}

impl Nom {
    pub fn new(position: Vec2, player_controlled: bool) -> Nom {
        Nom {
            size: if player_controlled {
                NOM_SPAWN_SIZE
            } else {
                thread_rng().gen_range(8..24) as f32
            },
            position,
            target_position: Vec2::ZERO,
            current_speed: 0.0, // Scalar speed
            orientation: 0.0,   // Initial orientation
            target_orientation: 0.0,
            max_speed: NOM_SPAWN_SPEED,
            acceleration: NOM_SPAWN_SPEED / 2.0,
            turning_speed: 180.0, // Degrees per second
            panicking: false,
            temp_is_colliding: false,
            player_controlled,
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
            self.update_grid_cell();
        }
    }

    pub fn draw(&self) {
        // Draw the circle representing the object
        draw_circle(
            self.position.x,
            self.position.y,
            self.size / 2.0,
            if self.temp_is_colliding && self.player_controlled {
                RED
            } else {
                GREEN
            },
        );

        // Draw the direction the object is facing (orientation line)
        let line_length = 30.0;
        let x2 = self.position.x + self.orientation.to_radians().cos() * line_length;
        let y2 = self.position.y + self.orientation.to_radians().sin() * line_length;
        draw_line(self.position.x, self.position.y, x2, y2, 2.0, RED);
    }

    pub fn update_position(&mut self, delta_time: &f32) {
        self.current_speed =
            (self.current_speed + self.acceleration * delta_time).min(self.max_speed);
        self.position.x += self.current_speed * self.orientation.to_radians().cos() * delta_time;
        self.position.y += self.current_speed * self.orientation.to_radians().sin() * delta_time;
    }

    pub fn update_grid_cell(&mut self) {
        let grid_cell_size: f32 = 30.0;
    }

    pub fn check_collision(&self, nom: &Nom) -> bool {
        if !self.bounding_box_collision(nom) {
            return false;
        }
        return self.circle_collision(nom);
    }

    pub fn bounding_box_collision(&self, nom: &Nom) -> bool {
        !(self.position.x + self.size / 2.0 < nom.position.x - nom.size / 2.0 || // Right of self.position is left of nom.position
        self.position.x - self.size / 2.0 > nom.position.x + nom.size / 2.0 || // Left of self.position is right of nom.position
        self.position.y + self.size / 2.0 < nom.position.y - nom.size / 2.0 || // Bottom of self.position is above nom.position
        self.position.y - self.size / 2.0 > nom.position.y + nom.size / 2.0) // Top of circle1 is below circle2
    }

    pub fn circle_collision(&self, nom: &Nom) -> bool {
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
