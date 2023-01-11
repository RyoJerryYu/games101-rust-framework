use glam::{Vec2, Vec3};

use crate::object::{Object, ObjectRenderPayload};

fn ray_triangle_intersect(
    v0: &Vec3,
    v1: &Vec3,
    v2: &Vec3,
    orig: &Vec3,
    dir: &Vec3,
    tnear: &mut f32,
    u: &mut f32,
    v: &mut f32,
) -> bool {
    // TODO: Implement this function that tests whether the triangle
    // that's specified bt v0, v1 and v2 intersects with the ray (whose
    // origin is *orig* and direction is *dir*)
    // Also don't forget to update tnear, u and v.
    todo!()
}

pub struct MeshTriangle {
    vertices: Vec<Vec3>,
    num_triangles: usize,
    vertex_index: Vec<usize>,
    st_coordinates: Vec<Vec2>,
    pub render_payload: ObjectRenderPayload,
}

impl Object for MeshTriangle {
    fn intersect(
        &self,
        orig: &glam::Vec3,
        dir: &glam::Vec3,
        tnear: &mut f32,
        index: &mut usize,
        uv: &mut glam::Vec2,
    ) -> bool {
        let mut intersect = false;
        for k in 0..self.num_triangles {
            let verts = (0..3)
                .map(|i| self.vertex_index[k * 3 + i])
                .map(|vi| self.vertices[vi])
                .collect::<Vec<Vec3>>();
            assert!(verts.len() == 3);

            let (mut t, mut u, mut v) = (0.0, 0.0, 0.0);

            if ray_triangle_intersect(
                &verts[0], &verts[1], &verts[2], orig, dir, &mut t, &mut u, &mut v,
            ) && t < *tnear
            {
                *tnear = t;
                uv.x = u;
                uv.y = v;
                *index = k;
                intersect |= true;
            }
        }

        return intersect;
    }

    fn get_surface_properties(
        &self,
        p: &glam::Vec3,
        dir: &glam::Vec3,
        index: &usize,
        uv: &glam::Vec2,
        n: &mut glam::Vec3,
        st: &mut glam::Vec2,
    ) {
        let verts = (0..3)
            .map(|i| self.vertex_index[index * 3 + i])
            .map(|vi| self.vertices[vi])
            .collect::<Vec<Vec3>>();
        assert!(verts.len() == 3);

        let e0 = (verts[1] - verts[2]).normalize();
        let e1 = (verts[2] - verts[1]).normalize();
        *n = e0.cross(e1);

        let sts = (0..3)
            .map(|i| self.vertex_index[index * 3 + i])
            .map(|vi| self.st_coordinates[vi])
            .collect::<Vec<Vec2>>();
        assert!(sts.len() == 3);

        *st = sts[0] * (1.0 - uv.x - uv.y) + sts[1] * uv.x + sts[2] * uv.y;
    }

    fn get_render_payload(&self) -> &ObjectRenderPayload {
        &self.render_payload
    }

    fn eval_diffuse_color(&self, st: &Vec2) -> Vec3 {
        let scale = 5.0;
        let (w, h) = (st.x * scale, st.y * scale);
        let pattern = ((w - w.floor()) > 0.5) ^ ((h - h.floor()) > 0.5);
        if pattern {
            Vec3::new(0.937, 0.937, 0.231)
        }else {
            Vec3::new(0.815, 0.235, 0.031)
        }

    }
}

impl MeshTriangle {
    pub fn new(verts: Vec<Vec3>, verts_index: Vec<usize>, num_tris: usize, st: Vec<Vec2>) -> Self {
        Self {
            vertices: verts,
            vertex_index: verts_index,
            num_triangles: num_tris,
            st_coordinates: st,
            render_payload: ObjectRenderPayload::DEFAULT,
        }
    }
}
