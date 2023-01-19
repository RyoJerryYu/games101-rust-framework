use glam::{Vec2, Vec3};

use crate::bounds3::Bounds3;
use crate::ray::Ray;

use crate::object::intersection::Intersection;

// representing the interfaces of virtual class `Object` in cpp codes
pub trait Object {
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
    fn sample(&self, pos: &mut Intersection, pdf: &mut f32);
    fn has_emit(&self) -> bool;
}
