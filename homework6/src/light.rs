use glam::Vec3;

pub trait Light {
    fn position(&self) -> Vec3;
    fn intensity(&self) -> Vec3;
}

pub struct PointLight {
    position: Vec3,
    intensity: Vec3,
}

impl Light for PointLight {
    fn position(&self) -> Vec3 {
        self.position
    }

    fn intensity(&self) -> Vec3 {
        self.intensity
    }
}

impl PointLight {
    pub fn new(p: &Vec3, i: f32) -> Self {
        return Self {
            position: *p,
            intensity: i * Vec3::ONE,
        };
    }
}
