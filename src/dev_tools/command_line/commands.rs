use macroquad::math::vec2;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nom::Nom;
use crate::quadtree::Quadtree;

pub fn handle_clear(noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>) {
    noms.borrow_mut().clear()
}

pub fn handle_spawn_nom(
    noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>,
    input: &str,
    invalid_command: &mut bool,
    quadtree: Rc<RefCell<Quadtree>>,
) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() == 2 {
        let mut nearby_noms: Vec<Rc<RefCell<Nom>>> = Vec::new();
        let new_nom = Nom::spawn_random();
        quadtree.borrow().retrieve(&mut nearby_noms, &new_nom);
        for nom in nearby_noms {
            // println!("{},{}", nom.borrow().position.x, nom.borrow().position.y);
        }
        let hoof = Rc::new(RefCell::new(new_nom));
        noms.borrow_mut().push(hoof.clone());
        quadtree.borrow_mut().insert(hoof.clone());
    } else if parts.len() == 3 {
        if let Ok(number) = parts[2].parse::<usize>() {
            if number >= 1 {
                for _ in 0..number {
                    let mut nearby_noms: Vec<Rc<RefCell<Nom>>> = Vec::new();
                    let new_nom = Nom::spawn_random();
                    quadtree.borrow().retrieve(&mut nearby_noms, &new_nom);
                    for nom in nearby_noms {
                        // println!("{},{}", nom.borrow().position.x, nom.borrow().position.y);
                    }
                    let hoof = Rc::new(RefCell::new(new_nom));
                    noms.borrow_mut().push(hoof.clone());
                    quadtree.borrow_mut().insert(hoof.clone());
                }
            } else {
                *invalid_command = true;
            }
        } else {
            *invalid_command = true;
        }
    } else {
        *invalid_command = true;
    }
}
