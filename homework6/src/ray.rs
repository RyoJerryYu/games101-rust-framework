use std::fmt::Display;

use glam::Vec3;

pub struct Ray {
    //Destination = origin + t*direction
    pub origin: Vec3,
    pub direction: Vec3,
    pub direction_inv: Vec3,
    pub t: f32, // transportation time
    pub t_min: f32,
    pub t_max: f32,
}

impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[origin:={}, direction={}, time={}]", self.origin, self.direction, self.t)
    }
}

impl Ray {
    pub fn new(ori: Vec3, dir: Vec3, t: f32) -> Self {
        Self {
            origin: ori,
            direction: dir,
            t,
            direction_inv: 1.0 / dir,
            t_min: 0.0,
            t_max: f32::MAX,
        }
    }

    // an alternative way for operator() in cpp
    pub fn on(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
