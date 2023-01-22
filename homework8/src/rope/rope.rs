use std::{collections::HashSet, rc::Rc};

use glam::Vec2;

use super::types::{Mass, Spring};

pub struct Rope {
    pub masses: Vec<Rc<Mass>>,
    pub springs: Vec<Spring>,
}

impl Rope {
    pub fn new(
        start: Vec2,
        end: Vec2,
        num_nodes: usize,
        node_mass: f32,
        k: f32,
        pinned_nodes: Vec<usize>,
    ) -> Self {
        assert!(num_nodes > 1, "rope less than 2 nodes is nonsense");
        for pinned in &pinned_nodes {
            assert!(pinned <= &num_nodes, "the {}th node is not exist", pinned);
        }
        let pinned_nodes: HashSet<_> = pinned_nodes.into_iter().collect();
        let position_diff = (end - start) / (num_nodes - 1) as f32;

        let mut res = Self {
            masses: vec![],
            springs: vec![],
        };

        // first mass
        let mut position = start;
        res.masses.push(Rc::new(Mass::new(
            position,
            node_mass,
            pinned_nodes.contains(&0),
        )));
        for i in 1..num_nodes {
            position = position + position_diff;
            res.masses.push(Rc::new(Mass::new(
                position,
                node_mass,
                pinned_nodes.contains(&i),
            )));
            res.springs
                .push(Spring::new(&res.masses[i - 1], &res.masses[i], k));
        }

        res
    }

    pub fn simulate_verlet(&mut self, delta_t: u32, gravity: Vec2) {}

    pub fn simulate_euler(&mut self, delta_t: u32, gravity: Vec2) {}
}
