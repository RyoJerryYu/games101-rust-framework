use std::ops::{Mul, Neg};

use crate::{
    bvh::BVHAccel,
    light::Light,
    object::{intersection::Intersection, material::MaterialType, object::Object},
    ray::Ray,
};
use glam::{Vec2, Vec3};

// used for resolve the ownership problem of objects
pub struct SceneObjectHolder {
    objects: Vec<Box<dyn Object>>,
}

impl SceneObjectHolder {
    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }
}

pub struct Scene {
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

    pub fn new_object_holder(&self) -> SceneObjectHolder {
        SceneObjectHolder { objects: vec![] }
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

    pub fn build_bvh(&mut self, object_holder: SceneObjectHolder) {
        println!("Scene build BVH start");
        self.bvh = Some(BVHAccel::new(object_holder.objects));
        println!("Scene build BVH end");
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

        // match m.get_type() {
        //     MaterialType::ReflectionAndRefraction => {
        //         let reflection_direction = reflect(ray.direction, n).normalize();
        //         let refraction_direction = refract(ray.direction, n, m.ior).normalize();

        //         // means getting the point over object surface, (while direction could face into object)
        //         // preventing cross the object again.
        //         let reflection_ray_orig = if reflection_direction.dot(n) < 0.0 {
        //             hit_point - n * self.epsilon
        //         } else {
        //             hit_point + n * self.epsilon
        //         };
        //         let refraction_ray_orig = if refraction_direction.dot(n) < 0.0 {
        //             hit_point - n * self.epsilon
        //         } else {
        //             hit_point + n * self.epsilon
        //         };

        //         let reflection_color = self.cast_ray(
        //             &Ray::new(reflection_ray_orig, reflection_direction),
        //             depth + 1,
        //         );
        //         let refraction_color = self.cast_ray(
        //             &Ray::new(refraction_ray_orig, refraction_direction),
        //             depth + 1,
        //         );

        //         let kr = fresnel(ray.direction, n, m.ior);
        //         hit_color = reflection_color * kr + refraction_color * (1.0 - kr);
        //     }
        //     MaterialType::Reflection => {
        //         let kr = fresnel(ray.direction, n, m.ior);
        //         let reflection_direction = reflect(ray.direction, n);

        //         // and here is wrong in the official homework6 code
        //         // I fixed it.
        //         let reflection_ray_orig = if reflection_direction.dot(n) < 0.0 {
        //             hit_point - n * self.epsilon
        //         } else {
        //             hit_point + n * self.epsilon
        //         };
        //         hit_color = self.cast_ray(
        //             &Ray::new(reflection_ray_orig, reflection_direction),
        //             depth + 1,
        //         ) * kr;
        //     }
        //     _ => {
        //         // [comment]
        //         // We use the Phong illumation model int the default case. The phong model
        //         // is composed of a diffuse and a specular reflection component.
        //         // [/comment]
        //         let mut light_amt: Vec3 = Vec3::ZERO;
        //         let mut specular_color: Vec3 = Vec3::ZERO;

        //         // ray.direction is facing to the surface,
        //         // and here the sign here is right.
        //         let shadow_point_orig = if ray.direction.dot(n) < 0.0 {
        //             hit_point + n * self.epsilon
        //         } else {
        //             hit_point - n * self.epsilon
        //         };

        //         // [comment]
        //         // Loop over all lights in the scene and sum their contribution up
        //         // We also apply the lambert cosine law
        //         // [/comment]
        //         for light in self.get_lights() {
        //             // do something with area light here for next assignment

        //             let light_dir = light.position() - hit_point;
        //             // square of the distance between hitPoint and the light
        //             let light_distance_2 = light_dir.dot(light_dir);
        //             let light_dir = light_dir.normalize();
        //             let l_dot_n = light_dir.dot(n).max(0.0);

        //             // is the point in shadow, and is the nearest occluding object closer to the object than the light itself?
        //             let in_shadow = self
        //                 .bvh
        //                 .as_ref()
        //                 .unwrap()
        //                 .intersect(&Ray::new(shadow_point_orig, light_dir));

        //             light_amt = match in_shadow {
        //                 Some(_) => light_amt + Vec3::ZERO,
        //                 None => light_amt + light.intensity() * l_dot_n,
        //             };

        //             let reflection_direction = reflect(-light_dir, n);
        //             specular_color += reflection_direction
        //                 .dot(ray.direction)
        //                 .neg()
        //                 .max(0.0)
        //                 .powf(m.specular_exponent)
        //                 .mul(light.intensity());
        //         }

        //         hit_color =
        //             light_amt * hit_object.eval_diffuse_color(&st) * m.kd + specular_color * m.ks;
        //     }
        // }

        hit_color
    }
}
