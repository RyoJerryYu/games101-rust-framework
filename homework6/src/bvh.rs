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

enum NodeContent {
    Leaf {
        object: Box<dyn Object>,
    },
    BiNode {
        left: Box<BVHBuildNode>,
        right: Box<BVHBuildNode>,
    },
}

struct BVHBuildNode {
    bounds: Bounds3,
    content: NodeContent,
}

impl BVHBuildNode {
    pub fn get_intersection(&self, ray: &Ray) -> Option<Intersection> {
        if !self.bounds.intersect_p(ray) {
            return None;
        }

        match &self.content {
            NodeContent::Leaf { object } => object.get_intersection(ray),
            NodeContent::BiNode { left, right } => {
                let (intersect_l, intersect_r) = (left.get_intersection(ray), right.get_intersection(ray));
                if intersect_l.is_none() {
                    return intersect_r; // some or none
                }

                if intersect_r.is_none() {
                    return intersect_l; // always some...
                }

                // both some, get the nearer one not behind origin
                let (intersect_l, intersect_r) = (intersect_l.unwrap(), intersect_r.unwrap());

                if intersect_l.distance < intersect_r.distance {
                    return Some(intersect_l);
                }
                return Some(intersect_r);
            },
        }
    }
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
                    content: NodeContent::Leaf {
                        object: objects.into_iter().nth(0).unwrap(),
                    },
                };
            }
            2 => {
                let mut left_objs = objects;
                let right_objs = left_objs.split_off(1);

                let left = BVHAccel::recursive_build(left_objs);
                let right = BVHAccel::recursive_build(right_objs);

                return BVHBuildNode {
                    bounds: left.bounds.union(&right.bounds),
                    content: NodeContent::BiNode {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
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
                    content: NodeContent::BiNode {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                };
            }
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.root.as_ref()?.get_intersection(ray)
    }
}
