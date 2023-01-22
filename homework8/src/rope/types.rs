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

pub struct Spring<'a> {
    pub k: f32,
    pub rest_length: f32,
    pub m1: &'a Mass,
    pub m2: &'a Mass,
}

impl<'a> Spring<'a> {
    pub fn new(a: &'a Mass, b: &'a Mass, k: f32) -> Self {
        Self {
            k: k,
            rest_length: (a.position - b.position).length(),
            m1: a,
            m2: b,
        }
    }
}
