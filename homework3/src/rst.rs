use crate::shader::{self, FragmentShader, Texture, VertexShader};

use glam::{Mat4, Vec3, Vec4};
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

    pub fn draw_triangle(&mut self, triangle_list: &Vec<utils::triangle::Triangle>) {
        let f1 = (50. - 0.1) / 2.0;
        let f2 = (50. + 0.1) / 2.0;

        let mvp = self.projection * self.view * self.model;

        for t in triangle_list {
            let mut newtri = t.clone();
            let vertex4 = t.to_vec4();
            // Vec4

            let mm = &[
                Vec4::from(self.view * self.model * vertex4[0]),
                Vec4::from(self.view * self.model * vertex4[1]),
                Vec4::from(self.view * self.model * vertex4[2]),
            ];

            let viewspace_pos = &[mm[0].truncate(), mm[1].truncate(), mm[2].truncate()];

            let v = &mut [mvp * vertex4[0], mvp * vertex4[1], mvp * vertex4[2]];

            for vec in v.iter_mut() {
                *vec = *vec / vec.w
            }

            let inv_trans = (self.view * self.model).inverse().transpose();
            let n = &[
                inv_trans * to_vec4(&t.normal[0], 0.),
                inv_trans * to_vec4(&t.normal[1], 0.),
                inv_trans * to_vec4(&t.normal[2], 0.),
            ];

            for vert in v.iter_mut() {
                vert.x = 0.5 * self.width as f32 * (vert.x + 1.);
                vert.y = 0.5 * self.width as f32 * (vert.y + 1.);
                vert.z = vert.z * f1 + f2;
            }

            for i in 0..3 {
                newtri.set_vertex(i, v[i].truncate());
            }

            for i in 0..3 {
                newtri.set_normal(i, n[i].truncate());
            }

            newtri.set_color(0, utils::triangle::Rgb(148, 121, 92));
            newtri.set_color(1, utils::triangle::Rgb(148, 121, 92));
            newtri.set_color(2, utils::triangle::Rgb(148, 121, 92));

            self.rasterize_triangle(newtri, *viewspace_pos);
        }
    }

    fn rasterize_triangle(&mut self, t: utils::triangle::Triangle, view_pos: [Vec3; 3]) {
        todo!()
        // TODO: From your HW3, get the triangle rasterization code.
        // TODO: Inside your rasterization loop:
        //    * v[i].w() is the vertex view space depth value z.
        //    * Z is interpolated view space depth for the current pixel
        //    * zp is depth between zNear and zFar, used for z-buffer

        // float Z = 1.0 / (alpha / v[0].w() + beta / v[1].w() + gamma / v[2].w());
        // float zp = alpha * v[0].z() / v[0].w() + beta * v[1].z() / v[1].w() + gamma * v[2].z() / v[2].w();
        // zp *= Z;

        // TODO: Interpolate the attributes:
        // auto interpolated_color
        // auto interpolated_normal
        // auto interpolated_texcoords
        // auto interpolated_shadingcoords

        // Use: fragment_shader_payload payload( interpolated_color, interpolated_normal.normalized(), interpolated_texcoords, texture ? &*texture : nullptr);
        // Use: payload.view_pos = interpolated_shadingcoords;
        // Use: Instead of passing the triangle's color directly to the frame buffer, pass the color to the shaders first to get the final color;
        // Use: auto pixel_color = fragment_shader(payload);
    }
}

fn to_vec4(v3: &Vec3, w: f32) -> Vec4 {
    Vec4::new(v3.x, v3.y, v3.z, w)
}
