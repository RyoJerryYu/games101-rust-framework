use crate::{object::{intersection::Intersection, object::Object}, ray::Ray};

pub struct BVHAccel {
    
}

impl BVHAccel {
    pub fn new(p: Vec<Box<dyn Object>>) -> Self {
        todo!()
    }
    pub fn intersect(&self, ray: &Ray) -> Intersection {
        todo!()
    }
}
