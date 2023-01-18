use glam::Vec3;

#[derive(Copy, Clone, Debug)]
pub enum MaterialType {
    DiffuseAndGlossy,
    ReflectionAndRefraction,
    Reflection,
}

#[derive(Clone, Debug)]
pub struct Material {
    pub m_type: MaterialType,
    pub m_color: Vec3,
    pub m_emission: Vec3,
    pub ior: f32,
    pub kd: f32,
    pub ks: f32,
    pub specular_exponent: f32,
}

impl Material {
    pub const DEFAULT: Self = Self::new_default();
    pub const fn new_default() -> Self {
        Self::new(MaterialType::DiffuseAndGlossy, Vec3::ONE, Vec3::ZERO)
    }

    pub const fn new(t: MaterialType, color: Vec3, emission: Vec3) -> Self {
        Self {
            m_type: t,
            m_color: color,
            m_emission: emission,

            ior: 0.0,
            kd: 0.0,
            ks: 0.0,
            specular_exponent: 0.0,
        }
    }

    pub fn get_type(&self) -> MaterialType {
        self.m_type
    }

    pub fn get_color(&self) -> Vec3 {
        self.m_color
    }

    pub fn get_emission(&self) -> Vec3 {
        self.m_emission
    }

    pub fn get_color_at(&self, _u: f32, _v: f32) -> Vec3 {
        Vec3::ZERO
    }
}
