use glam::Vec3;

use crate::object::{material::Material, object::Object};

pub struct Intersection<'a> {
    pub coords: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub obj: &'a dyn Object,
    pub m: &'a Material,
}

pub struct SampleResult {
    pub coords: Vec3, // pos.coords
    pub normal: Vec3, // pos.normal
    pub pdf: f32,
}