use glam::Vec3;

use crate::object::{MaterialType, Object, ObjectRenderPayload};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    pub render_payload: ObjectRenderPayload,
}

impl Object for Sphere {
    fn intersect(
        &self,
        orig: &glam::Vec3,
        dir: &glam::Vec3,
        tnear: &f32,
        index: &mut u32,
        uv: &mut glam::Vec2,
    ) -> bool {
        todo!()
    }

    fn get_surface_properties(
        &self,
        p: &glam::Vec3,
        pp: &glam::Vec3,
        index: &u32,
        uv: &glam::Vec2,
        n: &mut glam::Vec3,
        st: &mut glam::Vec2,
    ) {
        todo!()
    }

    fn get_render_payload(&self) -> &ObjectRenderPayload {
        &self.render_payload
    }

}

impl Sphere {
    pub fn new(c: &Vec3, r: f32) -> Self {
        Self {
            center: *c,
            radius: r,
            render_payload: ObjectRenderPayload::new(),
        }
    }
}
