use std::{collections::HashSet, iter::Map, rc::Rc};

use glam::Vec2;

use super::types::{Mass, Spring};

pub struct Rope {
    pub masses: Vec<Rc<Mass>>,
    pub springs: Vec<Spring>,

    masses_position: Vec<Vec2>,
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
        // TODO (Part 1): Create a rope starting at `start`, ending at `end`, and containing `num_nodes` nodes.

        //        Comment-in this part when you implement the constructor
        //        for (auto &i : pinned_nodes) {
        //            masses[i]->pinned = true;
        //        }
        assert!(num_nodes > 1, "rope less than 2 nodes is nonsense");
        for pinned in &pinned_nodes {
            assert!(pinned <= &num_nodes, "the {}th node is not exist", pinned);
        }
        let pinned_nodes: HashSet<_> = pinned_nodes.into_iter().collect();
        let position_diff = (end - start) / (num_nodes - 1) as f32;

        let mut res = Self {
            masses: vec![],
            springs: vec![],
            masses_position: vec![],
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

    pub fn simulate_verlet(&mut self, delta_t: f32, gravity: Vec2) {}

    // because of rust's super strong borrow check,
    // we can't modify a variable who have multiple reference,
    // thougth we know it's safety in single thread.
    //
    // so we use unsafe rust here and modify our mass fields by
    // casting an Rc to a mutable reference as follow:
    // ```
    // deref_mass(&m).position = Vec2::ZERO;
    // ```
    pub fn simulate_euler(&mut self, delta_t: f32, gravity: Vec2) {
        unsafe {
            let deref_mass = |m: &Rc<Mass>| -> &mut Mass {
                return (Rc::as_ptr(m) as *mut Mass).as_mut().unwrap();
            };

            for s in &self.springs {
                // TODO (Part 2): Use Hooke's law to calculate the force on a node

                let distence = s.m2.position - s.m1.position;
                // f represent force apply to m1
                let f = s.k * distence.normalize() * (distence.length() - s.rest_length);
                deref_mass(&s.m1).forces += f;
                deref_mass(&s.m2).forces += -f;
            }

            for m in &mut self.masses {
                let mut m = deref_mass(m);
                if !m.pinned {
                    // TODO (Part 2): Add the force due to gravity, then compute the new velocity and position
                    // TODO (Part 2): Add global damping
                    m.forces += gravity;
                    m.velocity *= f32::exp(-delta_t * 0.25);

                    let a = m.forces / m.mass;
                    let v_t1 = m.velocity + a * delta_t;
                    // let x_t1 = m.position + m.velocity * delta_t; // for explicit method
                    let x_t1 = m.position + v_t1 * delta_t; // for semi-implicit method

                    m.velocity = v_t1;
                    m.position = x_t1;
                }

                // Reset all forces on each mass
                m.forces = Vec2::ZERO;
            }
        }
    }

    pub fn masses_positions(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.masses.iter().map(|x| x.position)
    }

    // return [(start_posi, end_posi)]
    pub fn springs_position(&self) -> SpringsPosition {
        SpringsPosition::new(&self.springs)
    }
}

pub struct SpringsPosition<'a> {
    springs: &'a Vec<Spring>,
    index: usize,
}

impl Iterator for SpringsPosition<'_> {
    type Item = (Vec2, Vec2);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.springs.len() {
            let res = Some((
                self.springs[self.index].m1.position,
                self.springs[self.index].m2.position,
            ));
            self.index += 1;
            return res;
        }

        return None;
    }
}

impl<'a> SpringsPosition<'a> {
    fn new(springs: &'a Vec<Spring>) -> Self {
        Self { springs, index: 0 }
    }
}
