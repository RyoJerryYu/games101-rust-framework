
pub trait Rasterizable {
    fn data(&self) -> &[u8];
    fn size(&self) -> (u32, u32);
}