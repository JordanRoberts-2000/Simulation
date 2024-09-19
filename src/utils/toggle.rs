use super::draw::draw_rounded_rectangle;
use macroquad::prelude::*;

const DEFAULT_TOGGLE_WIDTH: f32 = 30.;
const DEFAULT_TOGGLE_HEIGHT: f32 = 10.;

pub struct ToggleSwitch {
    active: bool,
    position: Option<Vec2>,
    on_fn: Box<dyn Fn()>,
    off_fn: Box<dyn Fn()>,
}

impl ToggleSwitch {
    pub fn new(on_fn: Box<dyn Fn()>, off_fn: Box<dyn Fn()>) -> Self {
        Self {
            active: false,
            position: None,
            on_fn,
            off_fn,
        }
    }

    pub fn draw(&self) {
        if let Some(pos) = self.position {
            draw_rounded_rectangle(
                pos.x - DEFAULT_TOGGLE_WIDTH / 2.,
                pos.y - DEFAULT_TOGGLE_HEIGHT / 2.,
                DEFAULT_TOGGLE_WIDTH,
                DEFAULT_TOGGLE_HEIGHT,
                4.0,
                if self.active { BLUE } else { WHITE },
            );
            draw_circle(
                pos.x
                    + (if self.active {
                        DEFAULT_TOGGLE_WIDTH / 4.
                    } else {
                        -DEFAULT_TOGGLE_WIDTH / 4.
                    }),
                pos.y,
                DEFAULT_TOGGLE_HEIGHT * 0.85,
                GRAY,
            );
            draw_circle(
                pos.x
                    + (if self.active {
                        DEFAULT_TOGGLE_WIDTH / 4.
                    } else {
                        -DEFAULT_TOGGLE_WIDTH / 4.
                    }),
                pos.y,
                DEFAULT_TOGGLE_HEIGHT * 0.85 - 1.,
                WHITE,
            );
        }
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
        if self.active {
            (self.on_fn)();
        } else {
            (self.off_fn)()
        }
    }

    pub fn update(&mut self) {
        if let Some(pos) = self.position {
            let mouse_position = mouse_position(); // Get the mouse position

            let padding = 6.0; // Add padding to make the click area bigger

            // Check if the mouse is within the expanded toggle switch bounds
            let toggle_bounds = Rect::new(
                pos.x - (DEFAULT_TOGGLE_WIDTH / 2.) - padding,
                pos.y - (DEFAULT_TOGGLE_HEIGHT / 2.) - padding,
                DEFAULT_TOGGLE_WIDTH + 2.0 * padding,
                DEFAULT_TOGGLE_HEIGHT + 2.0 * padding,
            );

            // If the mouse is clicked and within bounds, toggle the state
            if is_mouse_button_pressed(MouseButton::Left)
                && toggle_bounds.contains(vec2(mouse_position.0, mouse_position.1))
            {
                self.toggle()
            }
        }
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.position = Some(vec2(x, y));
    }
}
