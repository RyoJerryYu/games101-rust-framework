use bitflags::bitflags;

pub trait Rasterizable {
    fn data(&self) -> &[u8];
    fn size(&self) -> (u32, u32);
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
    pub data: Vec<u8>,
}

impl Rasterizable for BufRasterizer {
    fn data(&self) -> &[u8] {
        return &self.data;
    }

    fn size(&self) -> (u32, u32) {
        return (self.width, self.height);
    }
}
