use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Bounds3 {
    pub min: Vec3,
    pub max: Vec3,
}

pub enum Dimension {
    X,
    Y,
    Z,
}

impl Bounds3 {
    pub fn new() -> Self {
        Self {
            min: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
            max: Vec3::new(f32::MIN, f32::MIN, f32::MIN),
        }
    }

    pub fn from_point(p: Vec3) -> Self {
        Self { min: p, max: p }
    }

    pub fn from_min_max(min: Vec3, max: Vec3) -> Self {
        let (min, max) = (min.min(max), min.max(max));
        Self { min, max }
    }

    pub fn union(&self, other: &Bounds3) -> Bounds3 {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    pub fn union_point(&self, p: Vec3) -> Bounds3 {
        Self {
            min: self.min.min(p),
            max: self.max.max(p),
        }
    }

    // the diagonal of the bounding box
    fn diagonal(&self) -> Vec3 {
        self.max - self.min
    }

    // the longest extented dimension
    pub fn max_extent(&self) -> Dimension {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z {
            Dimension::X
        } else if d.y > d.z {
            Dimension::Y
        } else {
            Dimension::Z
        }
    }

    // the center of the bounding box
    pub fn centroid(&self) -> Vec3 {
        (self.min + self.max) / 2.0
    }

    pub fn intersect_p(&self, ray: &crate::ray::Ray) -> bool {
        // invDir: ray direction(x,y,z), invDir=(1.0/x,1.0/y,1.0/z), use this because Multiply is faster that Division
        // dirIsNeg: ray direction(x,y,z), dirIsNeg=[int(x>0),int(y>0),int(z>0)], use this to simplify your logic
        // TODO test if ray bound intersects

        // improve: ray.direction_inv already exists, no need invDir arg, and we removed it
        //          and also we caculate dir_is_neg in this function

        let dir_is_neg: [bool; 3] = ray.direction.cmplt(Vec3::ZERO).into();

        // for each axis:
        // p = orig + t*dir
        // => px = origx + t*dirx
        // => t = (px - origx) / dirx
        // and intersect on bounding box, px = x_bound (which is max_x or min_x)
        // => t = (x_bound - origx) / dirx

        // for x,y,z axis, test if intersected t range exists
        // only intersecting range, not consider whether it is behind origin
        let (mut t_near, mut t_far) = (f32::MIN, f32::MAX);
        for i in 0..3 {
            let (axis_near, axis_far) = if dir_is_neg[i] {
                // if direct to negative, the greater plane is nearer to origin
                (self.max[i], self.min[i])
            } else {
                // dirct to positive, the minor plane is nearer
                (self.min[i], self.max[i])
            };

            t_near = t_near.max((axis_near - ray.origin[i]) * ray.direction_inv[i]);
            t_far = t_far.min((axis_far - ray.origin[i]) * ray.direction_inv[i]);

            if t_near > t_far {
                return false;
            }
        }

        // if the box is behind origin, it is not intersected.
        if t_far < 0.0 {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use crate::ray::Ray;

    use super::*;
    #[test]
    fn test_intersect_p() {
        let bound = Bounds3 {
            max: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            min: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        struct TestCase {
            ray: Ray,
            should_intersect: bool,
        }

        let cases = [
            TestCase {
                ray: Ray::new(Vec3::ZERO, Vec3::ONE),
                should_intersect: true,
            },
            TestCase {
                ray: Ray::new(Vec3::NEG_ONE, Vec3::NEG_ONE),
                should_intersect: false,
            },
            TestCase {
                ray: Ray::new(Vec3::NEG_X * 2.0, Vec3::ONE),
                should_intersect: false,
            }
        ];

        for tc in cases {
            assert_eq!(
                bound.intersect_p(&tc.ray),
                tc.should_intersect,
                "ray({}) intersect with bound({},{}) assert failed",
                tc.ray,
                bound.max,
                bound.min
            );
        }
    }
}
