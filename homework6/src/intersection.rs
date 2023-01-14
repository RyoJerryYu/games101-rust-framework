use glam::Vec3;

use crate::{material::Material, object::Object};

pub struct Intersection {
    pub happended: bool,
    pub coords: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub obj: Box<dyn Object>,
    pub m: Box<Material>,
}

// TODO
// impl new
