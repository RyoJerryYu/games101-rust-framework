use glam::Vec2;

use super::types::{Mass, Spring};

pub struct Rope<'a> {
    pub masses: Vec<Mass>,
    pub springs: Vec<Spring<'a>>,
}

impl<'a> Rope<'a> {
    pub fn new(
        start: Vec2,
        end: Vec2,
        num_nodes: usize,
        node_mass: f32,
        k: f32,
        pinned_nodes: Vec<usize>,
    ) -> Self {
        todo!()
    }

    pub fn simulate_verlet(&mut self, delta_t: u32, gravity: Vec2) {}

    pub fn simulate_euler(&mut self, delta_t: u32, gravity: Vec2) {}
}
