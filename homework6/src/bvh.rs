use glam::Vec3;

use crate::{
    bounds3::Bounds3,
    object::{intersection::Intersection, object::Object},
    ray::Ray,
};

enum BVHSplitMethod {
    NAIVE,
    SAH,
}

struct BVHBuildNode {
    bounds: Bounds3,
    left: Option<Box<BVHBuildNode>>,
    right: Option<Box<BVHBuildNode>>,
    object: Option<Box<dyn Object>>,
}

pub struct BVHAccel {
    maxPrimsInNode: usize,
    splitMethod: BVHSplitMethod,
    primitives: Vec<Box<dyn Object>>,
    root: Option<Box<BVHBuildNode>>,
}

impl BVHAccel {
    pub fn new(p: Vec<Box<dyn Object>>) -> Self {
        let mut res = Self {
            maxPrimsInNode: 1,
            splitMethod: BVHSplitMethod::NAIVE,
            primitives: p,
            root: None,
        };
        if res.primitives.len() == 0 {
            return res;
        }

        // start time
        res.root = Some(Box::new(BVHAccel::recursive_build(&res.primitives)));
        // end time
        // logging...

        return res;
    }

    fn recursive_build(objects: &Vec<Box<dyn Object>>) -> BVHBuildNode {
        todo!()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }
}
