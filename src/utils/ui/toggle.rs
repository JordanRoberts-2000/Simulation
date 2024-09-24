use crate::utils::draw::draw_rounded_rectangle;
use macroquad::prelude::*;

const DEFAULT_TOGGLE_WIDTH: f32 = 30.;
const DEFAULT_TOGGLE_HEIGHT: f32 = 10.;

pub struct Toggle {
    position: Vec2,
}

impl Toggle {
    // Initialize the toggle with a position
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: vec2(x, y),
        }
    }

    // Check for mouse input and update the provided mutable reference state
    pub fn update(&mut self, is_active: &mut bool) {
        let mouse_position = mouse_position(); // Get the mouse position

        let padding = 6.0; // Add padding to make the click area bigger

        // Define the clickable area (toggle bounds)
        let toggle_bounds = Rect::new(
            self.position.x - (DEFAULT_TOGGLE_WIDTH / 2.) - padding,
            self.position.y - (DEFAULT_TOGGLE_HEIGHT / 2.) - padding,
            DEFAULT_TOGGLE_WIDTH + 2.0 * padding,
            DEFAULT_TOGGLE_HEIGHT + 2.0 * padding,
        );

        // If the mouse is clicked and within bounds, toggle the state
        if is_mouse_button_pressed(MouseButton::Left)
            && toggle_bounds.contains(vec2(mouse_position.0, mouse_position.1))
        {
            *is_active = !*is_active; // Toggle the provided boolean reference
        }
    }

    // Draw the toggle switch based on the provided boolean reference
    pub fn draw(&self, is_active: &bool) {
        // Use the provided boolean reference to determine the toggle's appearance
        let color = if *is_active { BLUE } else { WHITE };
        draw_rounded_rectangle(
            self.position.x - DEFAULT_TOGGLE_WIDTH / 2.,
            self.position.y - DEFAULT_TOGGLE_HEIGHT / 2.,
            DEFAULT_TOGGLE_WIDTH,
            DEFAULT_TOGGLE_HEIGHT,
            4.0,
            color,
        );
        draw_circle(
            self.position.x
                + (if *is_active {
                    DEFAULT_TOGGLE_WIDTH / 4.
                } else {
                    -DEFAULT_TOGGLE_WIDTH / 4.
                }),
            self.position.y,
            DEFAULT_TOGGLE_HEIGHT * 0.85,
            GRAY,
        );
        draw_circle(
            self.position.x
                + (if *is_active {
                    DEFAULT_TOGGLE_WIDTH / 4.
                } else {
                    -DEFAULT_TOGGLE_WIDTH / 4.
                }),
            self.position.y,
            DEFAULT_TOGGLE_HEIGHT * 0.85 - 1.,
            WHITE,
        );
    }
}
