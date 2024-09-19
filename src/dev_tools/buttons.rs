use macroquad::prelude::*;

use crate::utils::button::Button;
use std::collections::HashMap;

use super::DevTools;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ButtonKeys {
    Clear,
    Freeze,
    Restart,
    SpawnOne,
    SpawnFive,
    SpawnTen,
    SpawnTwenty,
    SpawnFifty,
}

impl DevTools {
    pub fn create_buttons() -> HashMap<ButtonKeys, Button> {
        let mut buttons = HashMap::new();
        buttons.insert(
            ButtonKeys::Clear,
            Button::new(
                Box::new(|| {
                    println!("Button 1 clicked!");
                }),
                16,
                vec2(16.0, 8.0),
                BLACK,
                GRAY,
                WHITE,
                BLUE,
            ),
        );
        buttons.insert(
            ButtonKeys::Freeze,
            Button::new(
                Box::new(|| {
                    println!("Button 2 clicked!");
                }),
                16,
                vec2(16.0, 8.0),
                BLACK,
                GRAY,
                WHITE,
                BLUE,
            ),
        );
        buttons.insert(
            ButtonKeys::Restart,
            Button::new(
                Box::new(|| {
                    println!("Button 3 clicked!");
                }),
                16,
                vec2(16.0, 8.0),
                BLACK,
                GRAY,
                WHITE,
                BLUE,
            ),
        );
        buttons.insert(
            ButtonKeys::SpawnOne,
            Button::new(
                Box::new(|| {
                    println!("Spawn 1 clicked!");
                }),
                20,
                vec2(40.0, 8.0),
                BLACK,
                GRAY,
                WHITE,
                BLUE,
            ),
        );
        buttons.insert(
            ButtonKeys::SpawnFive,
            Button::new(
                Box::new(|| {
                    println!("Spawn 2 clicked!");
                }),
                20,
                vec2(16.0, 10.0),
                BLACK,
                GRAY,
                WHITE,
                BLUE,
            ),
        );
        buttons.insert(
            ButtonKeys::SpawnTen,
            Button::new(
                Box::new(|| {
                    println!("Spawn 3 clicked!");
                }),
                20,
                vec2(10.0, 10.0),
                BLACK,
                GRAY,
                WHITE,
                BLUE,
            ),
        );
        buttons.insert(
            ButtonKeys::SpawnTwenty,
            Button::new(
                Box::new(|| {
                    println!("Spawn 4 clicked!");
                }),
                20,
                vec2(10.0, 10.0),
                BLACK,
                GRAY,
                WHITE,
                BLUE,
            ),
        );
        buttons.insert(
            ButtonKeys::SpawnFifty,
            Button::new(
                Box::new(|| {
                    println!("Spawn 5 clicked!");
                }),
                20,
                vec2(10.0, 10.0),
                BLACK,
                GRAY,
                WHITE,
                BLUE,
            ),
        );
        buttons
    }

    pub fn set_button(&mut self, key: ButtonKeys, x: f32, y: f32, text: &str) {
        if let Some(button) = self.buttons.get_mut(&key) {
            button.set(x, y, text);
        } else {
            panic!("Button with key {:?} not found", key);
        }
    }
}
