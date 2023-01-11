use glam::{Vec2, Vec3};

pub enum MaterialType {
    DiffuseAndGlossy,
    ReflectionAndRefraction,
    Reflection,
}

// representing the structure of virtual class `Object` in cpp codes
// which containing public member variables that could set and get globaly.
pub struct ObjectRenderPayload {
    pub material_type: MaterialType,
    pub ior: f32,
    pub kd: f32,
    pub ks: f32,
    pub diffuse_color: Vec3,
    pub specular_exponent: f32,
}

impl ObjectRenderPayload {
    pub const DEFAULT: Self = Self::new();
    pub const fn new() -> Self {
        Self {
            material_type: MaterialType::DiffuseAndGlossy,
            ior: 1.3,
            kd: 0.8,
            ks: 0.2,
            diffuse_color: Vec3 {
                x: 0.2,
                y: 0.2,
                z: 0.2,
            },
            specular_exponent: 25.0,
        }
    }
}

// representing the interfaces of virtual class `Object` in cpp codes
pub trait Object {
    // intersect on orig + t * dir
    fn intersect(
        &self,
        orig: &Vec3,
        dir: &Vec3,
        tnear: &mut f32, // return t
        index: &mut usize,
        uv: &mut Vec2,
    ) -> bool;
    fn get_surface_properties(
        &self,
        p: &Vec3,
        pp: &Vec3,
        index: &usize,
        uv: &Vec2,
        n: &mut Vec3,
        st: &mut Vec2,
    );
    fn get_render_payload(&self) -> &ObjectRenderPayload;
    fn eval_diffuse_color(&self, _st: &Vec2) -> Vec3 {
        self.get_render_payload().diffuse_color
    }
}
