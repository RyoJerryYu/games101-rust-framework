use crate::shader::{self, FragmentShader, Texture, VertexShader};

use glam::Mat4;
pub use utils::rasterizer::{Buffers, IndBufId, PosBufId, Primitive, Rasterizable};

pub struct Rasterizer {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
    texture: Option<Texture>,

    frame_buf: Vec<utils::triangle::Rgb>,
    depth_buf: Vec<f32>,

    vertex_shader: Option<VertexShader>,
    fragment_shader: Option<FragmentShader>,

    width: u32,
    height: u32,
}

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
        let mut res = Self {
            model: Mat4::default(),
            view: Mat4::default(),
            projection: Mat4::default(),
            texture: None,

            frame_buf: Vec::new(),
            depth_buf: Vec::new(),

            vertex_shader: None,
            fragment_shader: None,

            width,
            height,
        };

        res.frame_buf
            .resize((width * height) as usize, utils::triangle::Rgb::default());
        res.depth_buf
            .resize((width * height) as usize, f32::INFINITY);
        res
    }

    pub fn set_model(&mut self, model: glam::Mat4) {
        self.model = model;
    }
    pub fn set_view(&mut self, view: glam::Mat4) {
        self.view = view;
    }
    pub fn set_projection(&mut self, projection: glam::Mat4) {
        self.projection = projection;
    }

    pub fn set_texture(&mut self, texture: shader::Texture) {
        self.texture = Some(texture);
    }

    pub fn set_vertex_shader(&mut self, shader: shader::VertexShader) {
        self.vertex_shader = Some(shader);
    }
    pub fn set_fragment_shader(&mut self, shader: shader::FragmentShader) {
        self.fragment_shader = Some(shader);
    }

    pub fn clear(&mut self, buffers: Buffers) {
        if buffers.contains(Buffers::COLOR) {
            self.frame_buf.fill(utils::triangle::Rgb(0, 0, 0));
        }
        if buffers.contains(Buffers::DEPTH) {
            self.depth_buf.fill(f32::INFINITY);
        }
    }

    pub fn draw(&mut self, primitive: Primitive, pos_buf: PosBufId, ind_buf: IndBufId) {
        todo!()
    }

    pub fn draw_triangle(&mut self, triangleList: &Vec<utils::triangle::Triangle>) {
        todo!()
    }
}
