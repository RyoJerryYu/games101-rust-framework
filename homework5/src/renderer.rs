use glam::Vec2;

use crate::{object, scene};

pub struct HitPayload {
    pub t_near: f32,
    pub index: u32,
    pub uv: Vec2,
    pub hit_obj: Box<dyn object::Object>,
}

pub struct Renderer {}

impl Renderer {
    pub fn render(&self, scene: &scene::Scene) {
        todo!()
    }
}
