use bitflags::bitflags;
use glam::Vec3;

use crate::rgb;

pub trait Rasterizable {
    fn data(&self) -> &Vec<rgb::Rgb>;
    fn size(&self) -> (u32, u32);
    fn u8_data(&self) -> Vec<u8> {
        rgb::rgb_vec_to_u8_slice(&self.data())
    }
}

bitflags! {
    pub struct Buffers: u8 {
        const COLOR = 0x1<<0;
        const DEPTH = 0x1<<1;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    Line,
    Triangle,
}

/*
 * For the curious: The draw function takes two buffer id's as its arguments.
 * These two structs make sure that if you mix up with their orders, the
 * compiler won't compile it. Aka : Type safety
 */
#[derive(Clone, Copy)]
pub struct PosBufId(pub u32);

#[derive(Clone, Copy)]
pub struct IndBufId(pub u32);

#[derive(Clone, Copy)]
pub struct ColBufId(pub u32);

/**
 * A simple implementation of Rasterizable trait.
 */
pub struct BufRasterizer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<rgb::Rgb>,
}

impl Rasterizable for BufRasterizer {
    fn data(&self) -> &Vec<rgb::Rgb> {
        &self.data
    }

    fn size(&self) -> (u32, u32) {
        return (self.width, self.height);
    }
}

impl BufRasterizer {
    pub fn from_vec3s(vec3s: Vec<Vec3>, width: u32, height: u32) -> Self {
        let data = vec3s.into_iter().map(|x| {
            rgb::Rgb::from(&(x * 255.0))
        }).collect();
        Self { width, height, data }
    }
}