use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Bounds3 {
    pub min: Vec3,
    pub max: Vec3,
}

impl Bounds3 {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        let (min, max) = (min.min(max), min.max(max));
        Self { min, max }
    }

    pub fn union(&self, other: &Bounds3) -> Bounds3 {
        todo!()
    }

    pub fn intersect(&self, ray: &crate::ray::Ray) -> bool {
        todo!()
    }
}
