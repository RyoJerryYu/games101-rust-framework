use crate::{light::Light, object::object::Object};
use glam::Vec3;

pub struct Scene {
    objects: Vec<Box<dyn Object>>,
    lights: Vec<Box<dyn Light>>,

    pub width: usize,
    pub height: usize,
    pub fov: f32,
    pub background_color: Vec3,
    pub max_depth: u32,
    pub epsilon: f32,
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

    pub fn many() {
        // many should be implement,
        // look back after impling ray
        todo!()
    }
}
