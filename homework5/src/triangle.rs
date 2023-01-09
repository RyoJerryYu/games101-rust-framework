use glam::{Vec2, Vec3};

use crate::object::{Object, ObjectRenderPayload};

pub struct MeshTriangle {
    vertices: Vec<Vec3>,
    num_triangles: u32,
    vertex_index: Vec<u32>,
    st_coordinates: Vec<Vec2>,
    pub render_payload: ObjectRenderPayload,
}

impl Object for MeshTriangle {
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

impl MeshTriangle {
    pub fn new(verts: Vec<Vec3>, verts_index: Vec<u32>, num_tris: u32, st: Vec<Vec2>) -> Self {
        Self {
            vertices: verts,
            vertex_index: verts_index,
            num_triangles: num_tris,
            st_coordinates: st,
            render_payload: ObjectRenderPayload::DEFAULT,
        }
    }
}
