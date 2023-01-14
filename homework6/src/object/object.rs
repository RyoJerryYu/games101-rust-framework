use glam::{Vec2, Vec3};

use crate::ray::Ray;

use crate::object::intersection::Intersection;

// representing the interfaces of virtual class `Object` in cpp codes
pub trait Object {
    // intersect on orig + t * dir
    // intersect(ray:&Ray) -> bool
    fn intersect(
        &self,
        ray: &Ray,
        tnear: &mut f32, // return t
        index: &mut usize,
        uv: &mut Vec2,
    ) -> bool;
    fn get_intersection(&self, ray: &Ray) -> Intersection;
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
}
