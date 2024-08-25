use piston_window::types::Color;
use piston_window::*;

use crate::draw::draw_detection_range;
use crate::food::Food;
use crate::nom::Nom;

const BACKGROUND_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

pub struct Simulation {
    nom_arr: Vec<Nom>,
    food_arr: Vec<Food>,
}

impl Simulation {
    pub fn new(starting_noms_amount: u64, starting_food_amount: u64) -> Simulation {
        let mut nom_vec: Vec<Nom> = Vec::new();
        for _i in 0..starting_noms_amount {
            nom_vec.push(Nom::new());
        }
        let mut food_vec: Vec<Food> = Vec::new();
        for _i in 0..starting_food_amount {
            food_vec.push(Food::new());
        }
        return Simulation {
            nom_arr: nom_vec,
            food_arr: food_vec,
        };
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        clear(BACKGROUND_COLOR, g);
        for food in &self.food_arr {
            Food::draw(food.position, &c, g);
        }
        for nom in &self.nom_arr {
            Nom::draw(nom.position, nom.size, &c, g);
        }
        draw_detection_range(300.0, 300.0, 80.0, [0.0, 0.0, 1.0, 1.0], &c, g);
    }
    // pub fn key_pressed(&mut self, key: Key) {
    //     let dir = match key {
    //         Key::Up => Some(Direction::Up),
    //         Key::Down => Some(Direction::Down),
    //         Key::Left => Some(Direction::Left),
    //         Key::Right => Some(Direction::Right),
    //         _ => Some(self.snake.head_direction()),
    //     };
    //     if let Some(dir) = dir {
    //         if dir == self.snake.head_direction().opposite() {
    //             return;
    //         }
    //     }
    // }
}
