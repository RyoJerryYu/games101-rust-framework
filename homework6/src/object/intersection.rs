use glam::Vec3;

use crate::object::{material::Material, object::Object};

pub struct Intersection {
    pub coords: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub obj: Box<dyn Object>,
    pub m: Box<Material>,
}


impl Intersection {
    // pub fn new() -> Self {
    //     Self {
    //         happended: false,
    //         coords: Vec3::ZERO,
    //         normal: Vec3::ZERO,
    //         distance: 0.0,
    //         obj: None,
    //         m: None,
    //     }
    // }
}