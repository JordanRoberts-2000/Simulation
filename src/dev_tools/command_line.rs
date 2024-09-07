use macroquad::prelude::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::nom::Nom;
use crate::utils::draw::draw_rounded_rectangle;

pub struct CommandLine {
    input_field: String,
    prev_commands: VecDeque<String>,
    prev_command_index: usize,
    invalid_command: bool,
}

impl CommandLine {
    pub fn new() -> CommandLine {
        return CommandLine {
            input_field: String::new(),
            prev_commands: VecDeque::with_capacity(4),
            prev_command_index: 0,
            invalid_command: false,
        };
    }

    pub fn draw(&self) {
        // draw_rectangle(10.0, screen_height() - 40.0, 300.0, 30.0, GRAY);
        draw_rounded_rectangle(10.0, screen_height() - 40.0, 300.0, 30.0, 5.0, DARKGRAY);
        if self.invalid_command {
            draw_rectangle_lines(10.0, screen_height() - 40.0, 300.0, 30.0, 6.0, RED);
        }
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
        if is_key_pressed(KeyCode::Up) {
            if !self.prev_commands.is_empty() {
                if self.prev_commands.len() == 1 {
                    self.input_field = self.prev_commands[0].clone();
                } else {
                    self.input_field = self.prev_commands[self.prev_command_index].clone();
                    if self.prev_command_index < self.prev_commands.len() - 1 {
                        self.prev_command_index += 1;
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Down) {
            if !self.prev_commands.is_empty() {
                // If there are commands and we are not at the first one
                if self.prev_command_index > 0 {
                    self.prev_command_index -= 1;
                    self.input_field = self.prev_commands[self.prev_command_index].clone();
                } else {
                    // If we are at the first command, clear the input field
                    self.input_field.clear();
                    self.invalid_command = false;
                }
            }
        }
        // letters into command line
        while let Some(ch) = get_char_pressed() {
            if ch.is_alphanumeric() || ch.is_whitespace() {
                self.input_field.push(ch);
            }
        }
    }

    pub fn store_command(&mut self, command: String) {
        let trimmed_command = command.trim().to_string();

        // If command is non-empty
        if !trimmed_command.is_empty() {
            // Check if the command already exists
            if let Some(pos) = self
                .prev_commands
                .iter()
                .position(|x| x == &trimmed_command)
            {
                // If it exists, remove it from its current position
                self.prev_commands.remove(pos);
            } else {
                // If we've reached 4 commands, remove the oldest one
                if self.prev_commands.len() == 4 {
                    self.prev_commands.pop_back();
                }
            }
            // Add the new command to the front
            self.prev_commands.push_front(trimmed_command);
        }
    }

    fn submit_command(&mut self, noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>) {
        self.invalid_command = false;
        match self.input_field.trim() {
            "clear" => noms.borrow_mut().clear(),
            "spawn nom" => {
                noms.borrow_mut()
                    .push(Rc::new(RefCell::new(Nom::spawn_random())));
            }
            // ToDo: unknown command visuals
            _ => self.invalid_command = true,
        }
        self.store_command(self.input_field.clone());
        self.input_field.clear();
        self.prev_command_index = 0;
    }

    pub fn clear_input(&mut self) {
        self.input_field.clear();
        self.prev_command_index = 0;
        self.invalid_command = false;
    }
}
