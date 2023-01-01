use crate::shader::{self, FragmentShader, FragmentShaderPayload, Texture, VertexShader};

use glam::{Mat4, Vec2, Vec3, Vec4};
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
        unsafe {
            std::slice::from_raw_parts(
                std::mem::transmute(self.frame_buf.as_ptr()),
                self.frame_buf.len() * 3,
            )
        }
    }

    fn size(&self) -> (u32, u32) {
        (self.width, self.height)
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

    fn set_pixel(&mut self, point: &Vec3, color: &utils::triangle::Rgb) {
        let (x_range, y_range) = (0..self.width, 0..self.height);
        let (x, y) = (point.x as u32, point.y as u32);
        if !(x_range.contains(&x) && y_range.contains(&y)) {
            return;
        }

        let ind = (self.height - 1 - y) * self.width + x;
        self.frame_buf[ind as usize] = *color;
    }

    pub fn clear(&mut self, buffers: Buffers) {
        if buffers.contains(Buffers::COLOR) {
            self.frame_buf.fill(utils::triangle::Rgb(0, 0, 0));
        }
        if buffers.contains(Buffers::DEPTH) {
            self.depth_buf.fill(f32::INFINITY);
        }
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
        // get the bounding box of the triangle
        let mut max_x = 0.0f32;
        let mut max_y = 0.0f32;
        let mut min_x = self.width as f32;
        let mut min_y = self.height as f32;

        for vertex in t.v {
            max_x = max_x.max(vertex.x);
            min_x = min_x.min(vertex.y);
            max_y = max_y.max(vertex.y);
            min_y = min_y.min(vertex.y);
        }

        for x in min_x as u32..max_x as u32 {
            for y in min_y as u32..max_y as u32 {
                let (xc, yc) = (x as f32 + 0.5, y as f32 + 0.5);
                if !inside_triangle(xc, yc, &t) {
                    continue;
                }

                let [alpha, beta, gama] = compute_barcentric_2d(xc, yc, t.v);
                let v = t.to_vec4();
                let barcentric = Vec3::new(alpha, beta, gama);
                let v_z = Vec3::new(v[0].z, v[1].z, v[2].z);
                let v_w = Vec3::new(v[0].w, v[1].w, v[2].w);
                let mut z_interpolated = (barcentric * v_z / v_w).dot(Vec3::ONE);
                let w_reciprocal = 1. / (barcentric / v_w).dot(Vec3::ONE);
                z_interpolated *= w_reciprocal;

                if z_interpolated < 0. {
                    continue;
                }
                let buf_ind = ((self.height - 1 - y) * self.width + x) as usize;
                if z_interpolated > self.depth_buf[buf_ind] {
                    continue;
                }

                let interpolated_color = alpha * Vec3::from(t.color[0])
                    + beta * Vec3::from(t.color[1])
                    + gama * Vec3::from(t.color[2]);

                let interpolated_normal =
                    alpha * t.normal[0] + beta * t.normal[1] + gama * t.normal[2];

                let interpolated_texcoords =
                    alpha * t.tex_coords[0] + beta * t.tex_coords[1] + gama * t.tex_coords[2];

                let interpolated_shadingcoords =
                    alpha * view_pos[0] + beta * view_pos[1] + gama * view_pos[2];

                let pixel_color = self.fragment_shader.unwrap()(&FragmentShaderPayload {
                    view_pos: interpolated_shadingcoords,
                    color: interpolated_color,
                    normal: interpolated_normal,
                    tex_coords: interpolated_texcoords,
                    texture: &self.texture,
                });

                let pixel_color = utils::triangle::Rgb::from(&pixel_color);

                // z_interpolated < self.depth_buf[buf_ind]
                self.set_pixel(&Vec3::new(xc, yc, z_interpolated), &pixel_color);
                self.depth_buf[buf_ind] = z_interpolated;
            }
        }
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

fn inside_triangle(xc: f32, yc: f32, t: &utils::triangle::Triangle) -> bool {
    let mut v = vec![];
    for vec in t.v {
        v.push(Vec3::new(vec.x, vec.y, 1.));
    }

    let f0 = v[1].cross(v[0]);
    let f1 = v[2].cross(v[1]);
    let f2 = v[0].cross(v[2]);

    let p = Vec3::new(xc, yc, 1.);

    return (p.dot(f0) * f0.dot(v[2]) > 0.)
        && (p.dot(f1) * f1.dot(v[0]) > 0.)
        && (p.dot(f2) * f2.dot(v[1]) > 0.);
}

fn compute_barcentric_2d(x: f32, y: f32, v: [Vec3; 3]) -> [f32; 3] {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y)
        / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y
            - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y)
        / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y
            - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y)
        / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y
            - v[1].x * v[0].y);
    [c1, c2, c3]
}
