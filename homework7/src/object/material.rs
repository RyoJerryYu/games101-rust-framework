use std::f32::consts::PI;

use glam::Vec3;

use crate::global::{self, get_random_float};

// i is the incident ray, n is the normalized normal
// i face to the surface, n face to the outside
// so i dot n is the projection of i on n, and is negative
// and i dot n * n is that projected length against n direction, so it's subtracted from i
// (result always facing to the outside, whatever n facing to.)
#[inline]
fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - 2.0 * i.dot(n) * n
}

// [comment]
// Compute refraction direction using Snell's law
//
// We need to handle with care the two possible situations:
//
//    - When the ray is inside the object
//
//    - When the ray is outside.
//
// If the ray is outside, you need to make cosi positive cosi = -N.I
//
// If the ray is inside, you need to invert the refractive indices and negate the normal N
// [/comment]
fn refract(i: Vec3, n: Vec3, ior: f32) -> Vec3 {
    // cosi means the angle between the incident ray and the normal
    // eta = 1/ior || eta = ior/1 , eta is the ratio of the refractive indices
    //     = sin(theta_t) / sin(theta_i)
    //     = IOR_out / IOR_in
    let mut cosi = i.dot(n).clamp(-1.0, 1.0);
    let (mut etai, mut etat) = (1.0, ior);
    let mut n = n;
    if cosi < 0.0 {
        cosi = -cosi;
    } else {
        (etai, etat) = (etat, etai);
        n = -n;
    }

    let eta = etai / etat;
    // k: the cosine^2 of the angle between the refracted ray and the normal
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        Vec3::ZERO
    } else {
        eta * i + (eta * cosi - k.sqrt()) * n
    }
}

// [comment]
// Compute Fresnel equation
//
// \param I is the incident view direction
//
// \param N is the normal at the intersection point
//
// \param ior is the material refractive index
// [/comment]
fn fresnel(i: Vec3, n: Vec3, ior: f32) -> f32 {
    let mut cosi = i.dot(n).clamp(-1.0, 1.0);
    let (mut etai, mut etat) = (1.0, ior);
    if cosi > 0.0 {
        (etai, etat) = (etat, etai);
    }
    // Compute sini using Snell's law
    let sint = etai / etat * (1.0 - cosi * cosi).max(0.0).sqrt();
    // Total internal reflection
    if sint >= 1.0 {
        return 1.0;
    }

    let cost = (1.0 - sint * sint).max(0.0).sqrt();
    cosi = cosi.abs();
    let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
    let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));
    return (rs * rs + rp * rp) / 2.0;
    // As a consequence of the conservation of energy, transmittance is given by:
    // kt = 1 - kr;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_reflect() {
        let i = Vec3::new(1.0, 1.0, -1.0);
        let n = Vec3::new(0.0, 0.0, 1.0);
        let r = super::reflect(i, n);
        assert_eq!(r, Vec3::new(1.0, 1.0, 1.0));
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MaterialType {
    Diffuse,
}

#[derive(Clone, Debug)]
pub struct Material {
    pub m_type: MaterialType,
    pub m_emission: Vec3,
    pub ior: f32,
    pub kd: Vec3,
    pub ks: Vec3,
    pub specular_exponent: f32,
}

impl Material {
    pub const fn new(t: MaterialType, emission: Vec3, kd: Vec3) -> Self {
        Self {
            m_type: t,
            m_emission: emission,

            ior: 0.0,
            kd: kd,
            ks: Vec3::ZERO,
            specular_exponent: 0.0,
        }
    }

    pub fn get_type(&self) -> MaterialType {
        self.m_type
    }

    pub fn get_emission(&self) -> Vec3 {
        self.m_emission
    }

    pub fn has_emission(&self) -> bool {
        return self.m_emission.length() > global::EPSILON;
    }

    // sample a ray by material properties
    pub fn sameple(&self, wi: Vec3, n: Vec3) -> Vec3 {
        match self.m_type {
            MaterialType::Diffuse => {
                // uniform sample on the hemisphere
                let (x1, x2) = (get_random_float(), get_random_float());
                let z = (1.0 - 2.0 * x1).abs(); // the z value of dir
                let r = (1.0 - z * z).sqrt(); // length of xy
                let phi = 2.0 * PI * x2; // dir of xy

                let local_ray = Vec3::new(r * phi.cos(), r * phi.sin(), z);
                to_world(local_ray, n)
            }
        }
    }

    // given a ray, calculate the PdF of this ray
    pub fn pdf(&self, wi: Vec3, wo: Vec3, n: Vec3) -> f32 {
        match self.m_type {
            MaterialType::Diffuse => {
                // uniform sample probability 1 / (2 * PI)
                if wo.dot(n) > 0.0 {
                    return 0.5 / PI;
                }
                return 0.0;
            }
        }
    }

    // given a ray, calculate the contribution of this ray
    pub fn eval(&self, wi: Vec3, wo: Vec3, n: Vec3) -> Vec3 {
        match self.m_type {
            MaterialType::Diffuse => {
                // calculate the contribution of diffuse   model
                let cosalpha = n.dot(wo);
                if cosalpha > 0.0 {
                    // diffuse
                    return self.kd / PI;
                }

                return Vec3::ZERO;
            }
        }
    }
}

// from local axises to world axises
fn to_world(a: Vec3, n: Vec3) -> Vec3 {
    let c = if n.x.abs() > n.y.abs() {
        let inv_len = 1.0 / (n.x * n.x + n.z * n.z).sqrt();
        Vec3::new(n.z * inv_len, 0.0, -n.x * inv_len)
    } else {
        let inv_len = 1.0 / (n.y * n.y + n.z * n.z).sqrt();
        Vec3::new(0.0, n.z * inv_len, -n.y * inv_len)
    };

    let b = c.cross(n);
    a.x * b + a.y * c + a.z * n
}
