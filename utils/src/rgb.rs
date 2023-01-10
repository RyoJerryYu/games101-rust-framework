
use glam::Vec3;
use std::ops::Add;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub const RED: Rgb = Rgb(255, 0, 0);
    pub const GREEN: Rgb = Rgb(0, 255, 0);
    pub const BLUE: Rgb = Rgb(0, 0, 255);

    #[inline]
    pub fn r(&self) -> u8 {
        self.0
    }
    #[inline]
    pub fn g(&self) -> u8 {
        self.1
    }
    #[inline]
    pub fn b(&self) -> u8 {
        self.2
    }
}

impl From<&image::Rgb<u8>> for Rgb {
    #[inline]
    fn from(i: &image::Rgb<u8>) -> Self {
        Self(i[0], i[1], i[2])
    }
}

impl From<&Vec3> for Rgb {
    #[inline]
    fn from(i: &Vec3) -> Self {
        let r = (i.x as u8).clamp(0, 255);
        let g = (i.y as u8).clamp(0, 255);
        let b = (i.z as u8).clamp(0, 255);

        Self(r, g, b)
    }
}

impl From<Rgb> for Vec3 {
    #[inline]
    fn from(rgb: Rgb) -> Self {
        Self {
            x: rgb.0 as f32,
            y: rgb.1 as f32,
            z: rgb.2 as f32,
        }
    }
}

impl Add<Rgb> for Rgb {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Rgb) -> Self::Output {
        Rgb(
            self.0.wrapping_add(rhs.0),
            self.1.wrapping_add(rhs.1),
            self.2.wrapping_add(rhs.2),
        )
    }
}

pub fn rgb_vec_to_u8_slice(buffer: &Vec<Rgb>) -> Vec<u8> {
    let mut res = vec![0_u8; buffer.len() * 3];
    for i in 0..buffer.len() {
        res[i * 3] = buffer[i].0;
        res[i * 3 + 1] = buffer[i].1;
        res[i * 3 + 2] = buffer[i].2;
    }
    res
}
