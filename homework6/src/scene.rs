use anyhow::{Ok, Result};
use std::ops::{Mul, Neg};

use crate::{
    bvh::BVHAccel,
    light::Light,
    object::{intersection::Intersection, material::MaterialType, object::Object},
    ray::Ray,
};
use glam::{Vec2, Vec3};

pub struct Scene {
    objects: Vec<Box<dyn Object>>,
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

    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    pub fn get_objects(&self) -> &Vec<Box<dyn Object>> {
        &self.objects
    }

    pub fn get_lights(&self) -> &Vec<Box<dyn Light>> {
        &self.lights
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.bvh.as_ref()?.intersect(ray)
    }

    pub fn build_bvh(&mut self) {
        todo!()
    }

    pub fn cast_ray(&self, ray: &Ray, depth: u32) -> Vec3 {
        if depth > self.max_depth {
            return Vec3::ZERO;
        }

        let intersection = match self.intersect(ray) {
            Some(i) => i,
            None => return self.background_color,
        };
        let m = intersection.m;
        let hit_object = intersection.obj;
        let mut hit_color = self.background_color;

        let uv: Vec2 = Vec2::ZERO;
        let index: usize = 0;
        let hit_point = intersection.coords;
        let mut n = intersection.normal;
        let mut st: Vec2 = Vec2::ZERO;
        hit_object.get_surface_properties(&hit_point, &ray.direction, &index, &uv, &mut n, &mut st);

        match m.get_type() {
            MaterialType::ReflectionAndRefraction => {
                let reflection_direction = reflect(ray.direction, n).normalize();
                let refraction_direction = refract(ray.direction, n, m.ior).normalize();

                let reflection_ray_orig = if reflection_direction.dot(n) < 0.0 {
                    hit_point - n * self.epsilon
                } else {
                    hit_point + n * self.epsilon
                };
                let refraction_ray_orig = if refraction_direction.dot(n) < 0.0 {
                    hit_point - n * self.epsilon
                } else {
                    hit_point + n * self.epsilon
                };

                let reflection_color = self.cast_ray(
                    &Ray::new(reflection_ray_orig, reflection_direction),
                    depth + 1,
                );
                let refraction_color = self.cast_ray(
                    &Ray::new(refraction_ray_orig, refraction_direction),
                    depth + 1,
                );

                let kr = fresnel(ray.direction, n, m.ior);
                hit_color = reflection_color * kr + refraction_color * (1.0 - kr);
            }
            MaterialType::Reflection => {
                let kr = fresnel(ray.direction, n, m.ior);
                let reflection_direction = reflect(ray.direction, n);
                let reflection_ray_orig = if reflection_direction.dot(n) < 0.0 {
                    hit_point + n * self.epsilon
                } else {
                    hit_point - n * self.epsilon
                };
                hit_color = self.cast_ray(
                    &Ray::new(reflection_ray_orig, reflection_direction),
                    depth + 1,
                ) * kr;
            }
            _ => {
                // [comment]
                // We use the Phong illumation model int the default case. The phong model
                // is composed of a diffuse and a specular reflection component.
                // [/comment]
                let mut light_amt: Vec3 = Vec3::ZERO;
                let mut specular_color: Vec3 = Vec3::ZERO;
                let shadow_point_orig = if ray.direction.dot(n) < 0.0 {
                    hit_point + n * self.epsilon
                } else {
                    hit_point - n * self.epsilon
                };

                // [comment]
                // Loop over all lights in the scene and sum their contribution up
                // We also apply the lambert cosine law
                // [/comment]
                for light in self.get_lights() {
                    // do something with area light here for next assignment

                    let light_dir = light.position() - hit_point;
                    // square of the distance between hitPoint and the light
                    let light_distance_2 = light_dir.dot(light_dir);
                    let light_dir = light_dir.normalize();
                    let l_dot_n = light_dir.dot(n).max(0.0);

                    // is the point in shadow, and is the nearest occluding object closer to the object than the light itself?
                    let in_shadow = self
                        .bvh
                        .as_ref()
                        .unwrap()
                        .intersect(&Ray::new(shadow_point_orig, light_dir));

                    light_amt = match in_shadow {
                        Some(_) => light_amt + Vec3::ZERO,
                        None => light_amt + light.intensity() * l_dot_n,
                    };

                    let reflection_direction = reflect(-light_dir, n);
                    specular_color += reflection_direction
                        .dot(ray.direction)
                        .neg()
                        .max(0.0)
                        .powf(m.specular_exponent)
                        .mul(light.intensity());
                }

                hit_color =
                    light_amt * hit_object.eval_diffuse_color(&st) * m.kd + specular_color * m.ks;
            }
        }

        hit_color
    }
}

// i is the incident ray, n is the normalized normal
// i face to the surface, n face to the outside
// so i dot n is the projection of i on n, and is negative
// and i dot n * n is that projected length against n direction, so it's subtracted from i
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_reflect() {
        let i = Vec3::new(1.0, 1.0, -1.0);
        let n = Vec3::new(0.0, 0.0, 1.0);
        let r = super::reflect(i, n);
        assert_eq!(r, Vec3::new(1.0, 1.0, 1.0));
    }
}
