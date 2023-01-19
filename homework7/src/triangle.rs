use std::rc::Rc;

use anyhow::Result;
use glam::{Vec2, Vec3};
use obj::load_obj;

use crate::{
    bounds3::Bounds3,
    bvh::BVHAccel,
    object::{
        intersection::{Intersection, SampleResult},
        material::{Material, MaterialType},
        object::Object,
    }, global::get_random_float,
};

#[derive(Debug, Clone)]
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
    pub area: f32,
    pub m: Material,
    bounding_box: Bounds3,
}

impl Object for Triangle {
    fn get_intersection(&self, ray: &crate::ray::Ray) -> Option<Intersection> {
        if ray.direction.dot(self.normal) > 0.0 {
            // ray casted from inner
            return None;
        }
        // P = (1-u-v)A + uB + vC = A + u(B-A) + v(C-A)
        // P = O + tD
        // => O-A = -tD + u(B-A) + v(C-A)
        // where B-A = e1 and C-A = e2

        // => O-A = [-D, e1, e2] dot [t, u, v]^T
        // => [t, u, v]^T = [-D, e1, e2]^-1 * (O-A)

        // solving the inv of matrix, could apply cramer's rule, then there is:
        // [t, u, v]^T = 1/det * [|T e1 e2|, |-D T e2|, |-D e1 T|]^T
        // where T = O-A

        // note:
        // |T e1 e2| = (T x e1) dot e2
        // |-D T e2| = (-D x T) dot e2 = (D x e2) dot T
        // |-D e1 T| = (-D x e1) dot T = (T x e1) dot D
        // so we could call (D x e2) is P and (T x e1) is Q and reuse them

        // it's fast and less memory usage, but hard to read :(

        let pvec = ray.direction.cross(self.e2); // D ✖️ e2
        let det = self.e1.dot(pvec); // det([-D, e1, e2])
        if det < 0.0001 {
            // e1 is on the plane of ray and e2,
            // a.k.a almost horizontal ray
            // or the angle between e1 and e2 is almost 0

            // in another hand: det close to zero, could not solve the equation
            return None;
        }

        let det_inv = 1.0 / det;
        let tvec = ray.origin - self.v0; // O-A
        let u = tvec.dot(pvec) * det_inv;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(self.e1);
        let v = ray.direction.dot(qvec) * det_inv;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t_temp = self.e2.dot(qvec) * det_inv;

        // TODO find ray triangle intersection
        if t_temp < 0.0 {
            // behind light
            return None;
        }

        let inter = Intersection {
            coords: ray.on(t_temp),
            normal: self.normal,
            distance: t_temp,
            obj: self,
            m: &self.m,
        };
        return Some(inter);

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
        *n = self.normal;
        // don't need to change st
    }

    fn eval_diffuse_color(&self, _st: &Vec2) -> Vec3 {
        Vec3::ONE * 0.5
    }

    fn get_bounds(&self) -> &Bounds3 {
        &self.bounding_box
    }

    fn get_area(&self) -> f32 {
        self.area
    }

    fn sample(&self) -> Option<SampleResult> {
        let x = get_random_float().sqrt();
        let y = get_random_float();

        Some(SampleResult{
            coords: self.v0 * (1.0 - x) + self.v1 * ( x * (1.0 - y)) + self.v2 * (x * y),
            normal: self.normal,
            pdf: 1.0 / self.area,
        })
    }

    fn has_emit(&self) -> bool {
        self.m.has_emission()
    }
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, m: Material) -> Self {
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        let normal = e1.cross(e2).normalize();
        let bounding_box = Bounds3::from_min_max(v0, v1).union_point(v2);
        let area = e1.cross(e2).length() * 0.5;
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
            area,
            m,
            bounding_box,
        }
    }
}

pub struct MeshTriangle {
    bounding_box: Bounds3,
    triangles: Vec<Rc<Triangle>>,
    bvh: BVHAccel,
    area: f32,
    m: Material,
}

impl MeshTriangle {
    pub fn new(filename: &str, mt: &Material) -> Result<Self> {
        let input = std::io::BufReader::new(std::fs::File::open(filename)?);
        let loadout: obj::Obj<obj::Position> = load_obj(input)?;
        dbg!("obj loaded");

        let mut triangles = vec![];
        let mut min_vert = Vec3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);
        let mut max_vert = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);

        for i in (0..loadout.indices.len()).step_by(3) {
            let mut face_vertices = vec![Vec3::ZERO;3];

            for j in 0..3 {
                // dbg!(i, j);
                let vertice = &loadout.vertices[loadout.indices[i + j] as usize];
                face_vertices[j] = Vec3::from_array(vertice.position) * 60.0;

                min_vert = min_vert.min(face_vertices[j]);
                max_vert = max_vert.max(face_vertices[j]);
            }

            triangles.push(Rc::new(Triangle::new(
                face_vertices[0],
                face_vertices[1],
                face_vertices[2],
                mt.clone(),
            )))
        }

        let bounding_box = Bounds3::from_min_max(min_vert, max_vert);

        let mut area = 0.0;
        let mut ptrs: Vec<Rc<dyn Object>> = vec![];
        for triangle in triangles.iter() {
            ptrs.push(triangle.clone());
            area += triangle.get_area();
        }

        println!("MeshTriangle build bvh start");
        let bvh = BVHAccel::new(ptrs);
        println!("MeshTriangle build bvh end");

        Ok(Self {
            bounding_box,
            triangles,
            bvh,
            area,
            m: mt.clone(),
        })
    }
}

impl Object for MeshTriangle {
    fn get_intersection(&self, ray: &crate::ray::Ray) -> Option<Intersection> {
        self.bvh.intersect(ray)
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
        // don't need, not called
        todo!()
    }

    fn eval_diffuse_color(&self, _st: &Vec2) -> Vec3 {
        todo!()
    }

    fn get_bounds(&self) -> &Bounds3 {
        &self.bounding_box
    }

    fn get_area(&self) -> f32 {
        self.area
    }

    fn sample(&self) -> Option<SampleResult> {
        self.bvh.sample()
    }

    fn has_emit(&self) -> bool {
        self.m.has_emission()
    }
}
