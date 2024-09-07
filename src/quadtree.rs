use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nom::Nom;

const MAX_OBJECTS: usize = 4;
const MAX_LEVELS: usize = 5;

pub struct Quadtree {
    level: usize,
    bounds: Rect,
    objects: Vec<Rc<RefCell<Nom>>>,
    nodes: Option<[Box<Quadtree>; 4]>,
}

impl Quadtree {
    pub fn new() -> Quadtree {
        return Quadtree {
            level: 0,
            bounds: Rect::new(0.0, 0.0, screen_width(), screen_height()),
            objects: Vec::new(),
            nodes: None,
        };
    }

    pub fn insert(&mut self, nom: Rc<RefCell<Nom>>) {
        // subdivisions already exist, put nom into one if possible
        if !self.nodes.is_none() {
            let index = self.get_index(&nom.borrow());
            if let Some(ref mut nodes) = self.nodes {
                if let Some(index) = index {
                    nodes[index].insert(nom.clone());
                    return;
                }
            }
        }

        // not enough noms in space or nom is too big / invalid position
        self.objects.push(nom.clone());

        // subdivide and put current noms into created quadrants
        if self.objects.len() >= MAX_OBJECTS && self.level < MAX_LEVELS {
            if self.nodes.is_none() {
                self.subdivide();
                let mut remaining_objects = Vec::new();
                while let Some(obj) = self.objects.pop() {
                    let obj_index = self.get_index(&obj.borrow());
                    if let Some(ref mut nodes) = self.nodes {
                        if let Some(index) = obj_index {
                            nodes[index].insert(obj.clone());
                        } else {
                            remaining_objects.push(obj);
                        }
                    }
                }
                self.objects = remaining_objects;
            }
        }
    }

    fn embedded_new(level: usize, bounds: Rect) -> Quadtree {
        Quadtree {
            level,
            bounds,
            objects: Vec::new(),
            nodes: None,
        }
    }

    pub fn draw(&self) {
        draw_rectangle_lines(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            self.bounds.h,
            1.0,
            DARKGRAY,
        );
        if let Some(ref nodes) = self.nodes {
            for node in nodes.iter() {
                node.draw();
            }
        }
    }

    fn subdivide(&mut self) {
        let sub_width = self.bounds.w / 2.0;
        let sub_height = self.bounds.h / 2.0;
        let x = self.bounds.x;
        let y = self.bounds.y;

        let section_1 = Rect::new(x, y, sub_width, sub_height);
        let section_2 = Rect::new(x + sub_width, y, sub_width, sub_height);
        let section_3 = Rect::new(x, y + sub_height, sub_width, sub_height);
        let section_4 = Rect::new(x + sub_width, y + sub_height, sub_width, sub_height);

        self.nodes = Some([
            Box::new(Quadtree::embedded_new(self.level + 1, section_1)),
            Box::new(Quadtree::embedded_new(self.level + 1, section_2)),
            Box::new(Quadtree::embedded_new(self.level + 1, section_3)),
            Box::new(Quadtree::embedded_new(self.level + 1, section_4)),
        ]);
    }

    // Determine which quadrant the object belongs to
    pub fn get_index(&self, nom: &Nom) -> Option<usize> {
        let vertical_midpoint = self.bounds.x + self.bounds.w / 2.0;
        let horizontal_midpoint = self.bounds.y + self.bounds.h / 2.0;
        let nom_radius = nom.size / 2.0;

        let in_top_quadrant = nom.position.y - nom_radius < horizontal_midpoint
            && nom.position.y + nom_radius < horizontal_midpoint;
        let in_bottom_quadrant = nom.position.y - nom_radius > horizontal_midpoint;

        if nom.position.x - nom_radius < vertical_midpoint
            && nom.position.x + nom_radius < vertical_midpoint
        {
            if in_top_quadrant {
                return Some(0); // Top-left quadrant
            } else if in_bottom_quadrant {
                return Some(2); // Bottom-left quadrant
            }
        } else if nom.position.x - nom_radius > vertical_midpoint {
            if in_top_quadrant {
                return Some(1); // Top-right quadrant
            } else if in_bottom_quadrant {
                return Some(3); // Bottom-right quadrant
            }
        }
        None
    }

    // Retrieve objects that could potentially collide with a given object
    // fn retrieve(&self, nom: &Nom, objects: &mut Vec<Nom>) {
    //     if let Some(ref nodes) = self.nodes {
    //         if let Some(index) = self.get_index(nom) {
    //             nodes[index].retrieve(nom, objects);
    //         }
    //     }

    //     // Add all objects from this node
    //     objects.extend(&self.objects);
    // }
}
