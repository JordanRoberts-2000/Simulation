use std::cell::RefCell;
use std::rc::Rc;

use crate::nom::{Nom, NomVariant};
use crate::quadtree::Quadtree;

pub fn handle_clear(noms: Rc<RefCell<Vec<Rc<RefCell<Nom>>>>>) {
    noms.borrow_mut().clear()
}

pub fn handle_freeze(movement: &mut bool) {
    *movement = false;
}

pub fn handle_unfreeze(movement: &mut bool) {
    *movement = true;
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
        let new_nom = Nom::spawn_random(NomVariant::Default);
        quadtree.borrow().retrieve(&mut nearby_noms, &new_nom);
        let mut no_collisions = true;
        for nom in nearby_noms {
            if new_nom.check_collision(&nom.borrow()) {
                no_collisions = false;
            }
        }
        if no_collisions {
            let hoof = Rc::new(RefCell::new(new_nom));
            noms.borrow_mut().push(hoof.clone());
            quadtree.borrow_mut().insert(hoof.clone());
        } else {
            println!("Could not spawn as collision would happen");
        }
    } else if parts.len() == 3 {
        if let Ok(number) = parts[2].parse::<usize>() {
            if number >= 1 {
                for _ in 0..number {
                    let mut nearby_noms: Vec<Rc<RefCell<Nom>>> = Vec::new();
                    let new_nom = Nom::spawn_random(NomVariant::Default);
                    quadtree.borrow().retrieve(&mut nearby_noms, &new_nom);
                    let mut no_collisions = true;
                    for nom in nearby_noms {
                        if new_nom.check_collision(&nom.borrow()) {
                            no_collisions = false;
                        }
                    }
                    if no_collisions {
                        let hoof = Rc::new(RefCell::new(new_nom));
                        noms.borrow_mut().push(hoof.clone());
                        quadtree.borrow_mut().insert(hoof.clone());
                    } else {
                        println!("Could not spawn as collision would happen");
                    }
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
