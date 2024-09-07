use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nom::Nom;
use crate::utils::draw::draw_rounded_rectangle;

pub struct CommandLine {
    input_field: String,
    focused: bool,
}

impl CommandLine {
    pub fn new() -> CommandLine {
        return CommandLine {
            input_field: String::new(),
            focused: false,
        };
    }

    pub fn draw(&self) {
        // draw_rectangle(10.0, screen_height() - 40.0, 300.0, 30.0, GRAY);
        draw_rounded_rectangle(10.0, screen_height() - 40.0, 300.0, 30.0, 5.0, DARKGRAY);
        draw_text(
            &self.input_field,
            110.0,
            screen_height() - 20.0,
            24.0,
            WHITE,
        );
        draw_text("Command:", 20.0, screen_height() - 20.0, 24.0, WHITE);
    }

    pub fn handle_inputs(&mut self, noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>) {
        if is_key_pressed(KeyCode::Enter) {
            self.submit_command(noms);
        }
        if is_key_pressed(KeyCode::Backspace) {
            self.input_field.pop();
        }
        // letters into command line
        while let Some(ch) = get_char_pressed() {
            if ch.is_alphanumeric() || ch.is_whitespace() {
                self.input_field.push(ch);
            }
        }
    }

    fn submit_command(&mut self, noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>) {
        match self.input_field.trim() {
            "clear" => noms.borrow_mut().clear(),
            "spawn nom" => {
                noms.borrow_mut()
                    .push(Rc::new(RefCell::new(Nom::new(vec2(800., 500.), false))));
            }
            // ToDo: unknown command visuals
            _ => println!("Command not found"),
        }
        self.input_field.clear();
    }

    pub fn clear_input(&mut self) {
        self.input_field.clear();
    }
}
