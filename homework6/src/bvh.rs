use glam::Vec3;

use crate::{
    bounds3::{Bounds3, Dimension},
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
    root: Option<Box<BVHBuildNode>>,
}

impl BVHAccel {
    pub fn new(p: Vec<Box<dyn Object>>) -> Self {
        let mut res = Self {
            maxPrimsInNode: 1,
            splitMethod: BVHSplitMethod::NAIVE,
            root: None,
        };
        if p.len() == 0 {
            return res;
        }

        // start time
        res.root = Some(Box::new(BVHAccel::recursive_build(p)));
        // end time
        // logging...

        return res;
    }

    fn recursive_build(objects: Vec<Box<dyn Object>>) -> BVHBuildNode {
        if objects.len() == 0 {
            panic!("logic error")
        }

        match objects.len() {
            1 => {
                return BVHBuildNode {
                    bounds: objects[0].get_bounds().clone(),
                    left: None,
                    right: None,
                    object: objects.into_iter().nth(0),
                };
            }
            2 => {
                let mut left_objs = objects;
                let right_objs = left_objs.split_off(1);

                let left = BVHAccel::recursive_build(left_objs);
                let right = BVHAccel::recursive_build(right_objs);

                return BVHBuildNode {
                    bounds: left.bounds.union(&right.bounds),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                    object: None,
                };
            }
            _ => {
                let mut centroid_bounds = Bounds3::new();
                for object in objects.iter() {
                    centroid_bounds = centroid_bounds.union_point(object.get_bounds().centroid());
                }

                let mut objects = objects;
                match centroid_bounds.max_extent() {
                    Dimension::X => objects.sort_by(|a, b| {
                        a.get_bounds()
                            .centroid()
                            .x
                            .partial_cmp(&b.get_bounds().centroid().x)
                            .unwrap()
                    }),
                    Dimension::Y => objects.sort_by(|a, b| {
                        a.get_bounds()
                            .centroid()
                            .y
                            .partial_cmp(&b.get_bounds().centroid().y)
                            .unwrap()
                    }),
                    Dimension::Z => objects.sort_by(|a, b| {
                        a.get_bounds()
                            .centroid()
                            .z
                            .partial_cmp(&b.get_bounds().centroid().z)
                            .unwrap()
                    }),
                }

                let mid = objects.len() / 2;

                let mut left_objs = objects;
                let right_objs = left_objs.split_off(mid);

                let left = BVHAccel::recursive_build(left_objs);
                let right = BVHAccel::recursive_build(right_objs);

                return BVHBuildNode {
                    bounds: left.bounds.union(&right.bounds),
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                    object: None,
                };
            }
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }
}
