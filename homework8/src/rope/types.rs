use std::rc::Rc;

use glam::Vec2;

pub struct Mass {
    pub mass: f32,
    pub pinned: bool,
    pub start_position: Vec2,
    pub position: Vec2,

    // explicit Verlet integration
    pub last_position: Vec2,

    // explicit Euler integration
    pub velocity: Vec2,
    pub forces: Vec2,
}

impl Mass {
    pub fn new(position: Vec2, mass: f32, pinned: bool) -> Self {
        Self {
            mass,
            pinned,
            start_position: position,
            position,
            last_position: position,
            velocity: Vec2::ZERO,
            forces: Vec2::ZERO,
        }
    }
}

pub struct Spring {
    pub k: f32,
    pub rest_length: f32,
    pub m1: Rc<Mass>,
    pub m2: Rc<Mass>,
}

impl Spring {
    pub fn new(a: &Rc<Mass>, b: &Rc<Mass>, k: f32) -> Self {
        Self {
            k: k,
            rest_length: (a.position - b.position).length(),
            m1: a.clone(),
            m2: b.clone(),
        }
    }
}
