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

    pub fn intersect(&self, ray: &crate::ray::Ray) -> bool {
        todo!()
    }
}
