use std::{
    ops::{Mul, Neg},
    rc::Rc,
};

use crate::{
    bvh::BVHAccel,
    global::get_random_float,
    light::Light,
    object::{intersection::Intersection, material::MaterialType, object::Object},
    ray::Ray,
};
use glam::{Vec2, Vec3};

pub struct Scene {
    objects: Vec<Rc<dyn Object>>,
    lights: Vec<Box<dyn Light>>,

    pub width: usize,
    pub height: usize,
    pub fov: f32,
    pub background_color: Vec3,
    pub max_depth: u32,
    pub epsilon: f32,

    bvh: Option<BVHAccel>,
}

impl Scene {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            objects: vec![],
            lights: vec![],
            width: w,
            height: h,
            fov: 90.0,
            background_color: Vec3 {
                x: 0.235294,
                y: 0.67451,
                z: 0.843137,
            },
            max_depth: 5,
            epsilon: 0.00001,

            bvh: None,
        }
    }

    pub fn add_object(&mut self, object: Rc<dyn Object>) {
        self.objects.push(object)
    }

    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    pub fn get_lights(&self) -> &Vec<Box<dyn Light>> {
        &self.lights
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.bvh.as_ref()?.intersect(ray)
    }

    pub fn build_bvh(&mut self) {
        println!("Scene build BVH start");
        self.bvh = Some(BVHAccel::new(self.objects.clone()));
        println!("Scene build BVH end");
    }

    fn sample_light(&self, pos: &mut Intersection, pdf: &mut f32) {
        let mut emit_area_sum = 0.0;

        // for each object has emit, add area
        let p = get_random_float() * emit_area_sum;
        emit_area_sum = 0.0;

        // for each object has emit, add area
        // when area > p, sample light on that object

        // means random choose an emitting object on the weight of area
        todo!()
    }
    // Implementation of Path Tracing
    pub fn cast_ray(&self, ray: &Ray, depth: u32) -> Vec3 {
        todo!()
        // TO DO Implement Path Tracing Algorithm here
    }
}

// i is the incident ray, n is the normalized normal
// i face to the surface, n face to the outside
// so i dot n is the projection of i on n, and is negative
// and i dot n * n is that projected length against n direction, so it's subtracted from i
// (result always facing to the outside, whatever n facing to.)
#[inline]
fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - 2.0 * i.dot(n) * n
}

// [comment]
// Compute refraction direction using Snell's law
//
// We need to handle with care the two possible situations:
//
//    - When the ray is inside the object
//
//    - When the ray is outside.
//
// If the ray is outside, you need to make cosi positive cosi = -N.I
//
// If the ray is inside, you need to invert the refractive indices and negate the normal N
// [/comment]
fn refract(i: Vec3, n: Vec3, ior: f32) -> Vec3 {
    // cosi means the angle between the incident ray and the normal
    // eta = 1/ior || eta = ior/1 , eta is the ratio of the refractive indices
    //     = sin(theta_t) / sin(theta_i)
    //     = IOR_out / IOR_in
    let mut cosi = i.dot(n).clamp(-1.0, 1.0);
    let (mut etai, mut etat) = (1.0, ior);
    let mut n = n;
    if cosi < 0.0 {
        cosi = -cosi;
    } else {
        (etai, etat) = (etat, etai);
        n = -n;
    }

    let eta = etai / etat;
    // k: the cosine^2 of the angle between the refracted ray and the normal
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        Vec3::ZERO
    } else {
        eta * i + (eta * cosi - k.sqrt()) * n
    }
}

// [comment]
// Compute Fresnel equation
//
// \param I is the incident view direction
//
// \param N is the normal at the intersection point
//
// \param ior is the material refractive index
// [/comment]
fn fresnel(i: Vec3, n: Vec3, ior: f32) -> f32 {
    let mut cosi = i.dot(n).clamp(-1.0, 1.0);
    let (mut etai, mut etat) = (1.0, ior);
    if cosi > 0.0 {
        (etai, etat) = (etat, etai);
    }
    // Compute sini using Snell's law
    let sint = etai / etat * (1.0 - cosi * cosi).max(0.0).sqrt();
    // Total internal reflection
    if sint >= 1.0 {
        return 1.0;
    }

    let cost = (1.0 - sint * sint).max(0.0).sqrt();
    cosi = cosi.abs();
    let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
    let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));
    return (rs * rs + rp * rp) / 2.0;
    // As a consequence of the conservation of energy, transmittance is given by:
    // kt = 1 - kr;
}
