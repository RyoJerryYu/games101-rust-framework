use glam::{Mat3, Vec2, Vec3};

use crate::object::{object::Object, material::Material};

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

    // intersect -> the point that ray cross the plane is in the triangle
    // for i in 0..2:
    //   n_plane dot (v[i] - p) = 0
    //   => n_plane dot (v[i] - orig - t * dir) = 0
    //   => n_plane dot (v[i] - orig) = t * n_plane dot dir
    //   => t = n_plane dot (v[i] - orig) / n_plane dot dir
    // where n_plane is a cross product for edges

    // second method:
    // p = (1 - a - b)*v[0] + a*v[1] + b*v[2]
    //   = orig + t * dir
    // =>
    // a*(v[1] - v[0]) + b*(v[2]-v[0]) + t*(-dir) = orig - v[0]
    // [(v[1]-v[0]), (v[2]-v[0], -dir)] dot [a,b,t]T = (orig - v[0])
    // [a,b,t]T = ([(v[1]-v[0]), (v[2]-v[0], -dir)])^-1 * (orig-v[0])

    // mat * [a,b,t]T = (orig - v[0])
    let mat = Mat3::from_cols(*v1 - *v0, *v2 - *v0, -*dir);
    let Vec3 { x: a, y: b, z: t } = mat.inverse() * (*orig - *v0);

    if a > 0.0 && b > 0.0 && (1.0 - a - b) > 0.0 && t > 0.0 {
        // well, according to other codes, u,v is the barycentric coordinate
        *tnear = t;
        *u = a;
        *v = b;
        return true;
    }

    return false;
}

pub struct Triangle {
    // counter-clockwise order
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,

    // edges
    pub e1: Vec3, // v1 - v0
    pub e2: Vec3, // v2 - v0

    // texture coordinates
    pub t0: Vec3,
    pub t1: Vec3,
    pub t2: Vec3,

    pub normal: Vec3,
    pub m: Option<Material>
}

impl Object for Triangle {
    fn intersect(
        &self,
        ray: &crate::ray::Ray,
        tnear: &mut f32, // return t
        index: &mut usize,
        uv: &mut Vec2,
    ) -> bool {
        todo!()
    }

    fn get_intersection(&self, ray: &crate::ray::Ray) -> crate::object::intersection::Intersection {
        todo!()
    }

    fn get_surface_properties(
        &self,
        p: &Vec3,
        dir: &Vec3,
        index: &usize,
        uv: &Vec2,
        n: &mut Vec3,
        st: &mut Vec2,
    ) {
        todo!()
    }

    fn eval_diffuse_color(&self, _st: &Vec2) -> Vec3 {
        todo!()
    }
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, m: Option<Material>) -> Self {
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        let normal = e1.cross(e2).normalize();
        Self {
            v0,
            v1,
            v2,
            e1,
            e2,
            t0: Vec3::ZERO,
            t1: Vec3::ZERO,
            t2: Vec3::ZERO,
            normal,
            m
        }
    }
}

pub struct MeshTriangle {
    vertices: Vec<Vec3>,
    num_triangles: usize,
    vertex_index: Vec<usize>,
    st_coordinates: Vec<Vec2>,
}

impl MeshTriangle {
    pub fn new(filename: &str) -> Self {
        todo!()
    }
}

impl Object for MeshTriangle {
    fn intersect(
        &self,
        ray: &crate::ray::Ray,
        tnear: &mut f32, // return t
        index: &mut usize,
        uv: &mut Vec2,
    ) -> bool {
        todo!()
    }

    fn get_intersection(&self, ray: &crate::ray::Ray) -> crate::object::intersection::Intersection {
        todo!()
    }

    fn get_surface_properties(
        &self,
        p: &Vec3,
        dir: &Vec3,
        index: &usize,
        uv: &Vec2,
        n: &mut Vec3,
        st: &mut Vec2,
    ) {
        todo!()
    }

    fn eval_diffuse_color(&self, _st: &Vec2) -> Vec3 {
        todo!()
    }
    // fn intersect(
    //     &self,
    //     orig: &glam::Vec3,
    //     dir: &glam::Vec3,
    //     tnear: &mut f32,
    //     index: &mut usize,
    //     uv: &mut glam::Vec2,
    // ) -> bool {
    //     let mut intersect = false;
    //     for k in 0..self.num_triangles {
    //         let verts = (0..3)
    //             .map(|i| self.vertex_index[k * 3 + i])
    //             .map(|vi| self.vertices[vi])
    //             .collect::<Vec<Vec3>>();
    //         assert!(verts.len() == 3);

    //         let (mut t, mut u, mut v) = (0.0, 0.0, 0.0);

    //         if ray_triangle_intersect(
    //             &verts[0], &verts[1], &verts[2], orig, dir, &mut t, &mut u, &mut v,
    //         ) && t < *tnear
    //         {
    //             *tnear = t;
    //             uv.x = u;
    //             uv.y = v;
    //             *index = k;
    //             intersect |= true;
    //         }
    //     }

    //     return intersect;
    // }

    // fn get_surface_properties(
    //     &self,
    //     p: &glam::Vec3,
    //     dir: &glam::Vec3,
    //     index: &usize,
    //     uv: &glam::Vec2,
    //     n: &mut glam::Vec3,
    //     st: &mut glam::Vec2,
    // ) {
    //     let verts = (0..3)
    //         .map(|i| self.vertex_index[index * 3 + i])
    //         .map(|vi| self.vertices[vi])
    //         .collect::<Vec<Vec3>>();
    //     assert!(verts.len() == 3);

    //     let e0 = (verts[1] - verts[0]).normalize();
    //     let e1 = (verts[2] - verts[1]).normalize();
    //     *n = e0.cross(e1).normalize();

    //     let sts = (0..3)
    //         .map(|i| self.vertex_index[index * 3 + i])
    //         .map(|vi| self.st_coordinates[vi])
    //         .collect::<Vec<Vec2>>();
    //     assert!(sts.len() == 3);

    //     *st = sts[0] * (1.0 - uv.x - uv.y) + sts[1] * uv.x + sts[2] * uv.y;
    // }

    // fn get_render_payload(&self) -> &ObjectRenderPayload {
    //     &self.render_payload
    // }

    // fn eval_diffuse_color(&self, st: &Vec2) -> Vec3 {
    //     let scale = 5.0;
    //     let (w, h) = (st.x * scale, st.y * scale);
    //     let pattern = ((w - w.floor()) > 0.5) ^ ((h - h.floor()) > 0.5);
    //     if pattern {
    //         Vec3::new(0.937, 0.937, 0.231)
    //     } else {
    //         Vec3::new(0.815, 0.235, 0.031)
    //     }
    // }
}
