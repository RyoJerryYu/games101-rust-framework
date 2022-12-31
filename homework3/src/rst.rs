use crate::shader;

pub use utils::rasterizer::{Buffers, IndBufId, PosBufId, Primitive, Rasterizable};

pub struct Rasterizer {}

impl utils::rasterizer::Rasterizable for Rasterizer {
    fn data(&self) -> &[u8] {
        todo!()
    }

    fn size(&self) -> (u32, u32) {
        todo!()
    }
}

impl Rasterizer {
    pub fn new(width: u32, height: u32) -> Self {
        todo!()
    }

    pub fn set_model(&mut self, model: glam::Mat4) {
        todo!()
    }
    pub fn set_view(&mut self, view: glam::Mat4) {
        todo!()
    }
    pub fn set_projection(&mut self, projection: glam::Mat4) {
        todo!()
    }

    pub fn set_texture(&mut self, texture: shader::Texture) {
        todo!()
    }

    pub fn set_vertex_shader(&mut self, shader: shader::VertexShader) {
        todo!()
    }
    pub fn set_fragment_shader(&mut self, shader: shader::FragmentShader) {
        todo!()
    }

    pub fn clear(&mut self, buffers: Buffers) {
        todo!()
    }

    pub fn draw(&mut self, primitive: Primitive, pos_buf: PosBufId, ind_buf: IndBufId) {
        todo!()
    }

    pub fn draw_triangle(&mut self, triangleList: &Vec<utils::triangle::Triangle>) {
        todo!()
    }
}
