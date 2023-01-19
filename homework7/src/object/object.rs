use glam::{Vec2, Vec3};

use crate::bounds3::Bounds3;
use crate::ray::Ray;

use crate::object::intersection::Intersection;

use super::intersection::SampleResult;

// representing the interfaces of virtual class `Object` in cpp codes
pub trait Object: Send + Sync {
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection>;
    fn get_surface_properties(
        &self,
        p: &Vec3,
        dir: &Vec3,
        index: &usize,
        uv: &Vec2,
        n: &mut Vec3,
        st: &mut Vec2,
    );
    fn eval_diffuse_color(&self, _st: &Vec2) -> Vec3;
    fn get_bounds(&self) -> &Bounds3;
    fn get_area(&self) -> f32;
    // return pos: Intersection, pdf: f32
    fn sample(&self) -> Option<SampleResult>;
    fn has_emit(&self) -> bool;
}
