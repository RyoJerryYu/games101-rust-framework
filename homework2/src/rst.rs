use std::collections::HashMap;

use bitflags::bitflags;
use glam::{IVec2, Mat4, Vec2, Vec3, Vec4};
use homework2::inside_triangle;

use homework2::triangle::{Rgb, Triangle};

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
pub struct PosBufId(u32);

#[derive(Clone, Copy)]
pub struct IndBufId(u32);

#[derive(Clone, Copy)]
pub struct ColBufId(u32);

pub struct Rasterizer {
    model: Mat4,
    view: Mat4,
    projection: Mat4,

    pos_buf: HashMap<u32, Vec<Vec3>>,
    ind_buf: HashMap<u32, Vec<[usize; 3]>>,
    col_buf: HashMap<u32, Vec<Rgb>>,

    frame_buf: Vec<Rgb>,
    depth_buf: Vec<f32>,

    width: u32,
    height: u32,
    next_id: u32,
}

impl Rasterizer {
    pub fn new(w: u32, h: u32) -> Self {
        let mut res = Self {
            model: Mat4::default(),
            view: Mat4::default(),
            projection: Mat4::default(),
            pos_buf: HashMap::default(),
            ind_buf: HashMap::default(),
            col_buf: HashMap::default(),
            frame_buf: Vec::new(),
            depth_buf: Vec::default(),
            width: w,
            height: h,
            next_id: 0,
        };
        res.frame_buf.resize((w * h) as usize, Rgb::default());
        res.depth_buf.resize((w * h) as usize, f32::INFINITY);
        res
    }

    pub fn load_positions(&mut self, positions: Vec<Vec3>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions);
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: Vec<[usize; 3]>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices);
        IndBufId(id)
    }

    pub fn load_colors(&mut self, colors: Vec<Vec3>) -> ColBufId {
        let id = self.get_next_id();
        let rgbs = colors.iter().map(|c| Rgb::from(c)).collect();
        self.col_buf.insert(id, rgbs);
        ColBufId(id)
    }

    pub fn set_model(&mut self, model: Mat4) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Mat4) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Mat4) {
        self.projection = projection;
    }

    pub fn set_pixel(&mut self, point: &Vec3, color: &Rgb) {
        if point.x < 0.0
            || point.x as u32 >= self.width
            || point.y < 0.0
            || point.y as u32 >= self.height
        {
            return;
        }
        let ind = (self.height - 1 - point.y as u32) * self.width + point.x as u32;
        self.frame_buf[ind as usize] = *color;
    }

    pub fn clear(&mut self, buffers: Buffers) {
        if buffers.contains(Buffers::COLOR) {
            self.frame_buf.fill(Rgb::default());
        }
        if buffers.contains(Buffers::DEPTH) {
            self.depth_buf.fill(f32::INFINITY);
        }
    }

    pub fn draw(
        &mut self,
        pos_buf_id: PosBufId,
        ind_buf_id: IndBufId,
        col_buf_id: ColBufId,
        typ: Primitive,
    ) {
        if typ != Primitive::Triangle {
            unimplemented!()
        }

        let buf = self.pos_buf.get(&pos_buf_id.0).unwrap().clone();
        let ind = self.ind_buf.get(&ind_buf_id.0).unwrap().clone();
        let col = self.col_buf.get(&col_buf_id.0).unwrap().clone();

        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;

        let mvp = self.projection * self.view * self.model;
        for i in ind {
            let mut t = Triangle::new();
            let mut v = [
                mvp * to_vec4(buf[i[0]], 1.0),
                mvp * to_vec4(buf[i[1]], 1.0),
                mvp * to_vec4(buf[i[2]], 1.0),
            ];

            // Homogeneus division
            for vi in v.iter_mut() {
                *vi /= vi.w;
            }

            // Viewport transformation
            for vert in v.iter_mut() {
                vert.x = self.width as f32 * (vert.x + 1.0) * 0.5;
                vert.y = self.height as f32 * (vert.y + 1.0) * 0.5;
                vert.z = vert.z * f1 + f2;
            }

            for vert_ind in 0..3 {
                t.set_vertex(vert_ind, Vec3::new(v[vert_ind].x, v[vert_ind].y, v[vert_ind].z));
                t.set_color(vert_ind, col[i[vert_ind]]);
            }

            self.rasterize_triangle(&t);
        }
    }

    //Screen space rasterization
    fn rasterize_triangle(&mut self, t: &Triangle) {
        // get the bounding box of the triangle
        let mut max_x = 0.0f32;
        let mut max_y = 0.0f32;
        let mut min_x = self.width as f32;
        let mut min_y = self.height as f32;

        for vertex in t.v {
            if vertex.x > max_x {
                max_x = vertex.x;
            }
            if vertex.x < min_x {
                min_x = vertex.x;
            }
            if vertex.y > max_y {
                max_y = vertex.y;
            }
            if vertex.y < min_y {
                min_y = vertex.y;
            }
        }

        for x in min_x as u32..max_x as u32 {
            for y in min_y as u32..max_y as u32 {
                // the center of the pixel
                let (xc, yc) = (x as f32 + 0.5, y as f32 + 0.5);
                if !inside_triangle(xc, yc, t) {
                    continue;
                }

                // get the interpolated z value
                let [alpha, beta, gama] = compute_barcentric_2d(xc, yc, t.v);
                let v = t.to_vec4();
                let w_reciprocal = 1.0 / (alpha / v[0].w + beta / v[1].w + gama / v[2].w);
                let mut z_interpolated =
                    alpha * v[0].z / v[0].w + beta * v[1].z / v[1].w + gama * v[2].z / v[2].w;
                z_interpolated *= w_reciprocal;

                if z_interpolated < 0.0 {
                    continue;
                }
                let buf_ind = ((self.height - 1 - y) * self.width + x) as usize;
                if z_interpolated < self.depth_buf[buf_ind] {
                    self.set_pixel(&Vec3::new(xc, yc, z_interpolated), &t.get_color());
                    self.depth_buf[buf_ind] = z_interpolated;
                }
            }
        }
    }

    fn draw_line(&mut self, begin: Vec3, end: Vec3, line_color: Rgb) {
        let x1 = begin.x;
        let y1 = begin.y;
        let x2 = end.x;
        let y2 = end.y;

        let dx = (x2 - x1) as i32;
        let dy = (y2 - y1) as i32;
        let dx1 = dx.abs();
        let dy1 = dy.abs();
        let mut px = 2 * dy1 - dx1;
        let mut py = 2 * dx1 - dy1;

        let (mut x, mut y, xe, ye): (i32, i32, i32, i32);

        if dy1 <= dx1 {
            if dx >= 0 {
                x = x1 as i32;
                y = y1 as i32;
                xe = x2 as i32;
            } else {
                x = x2 as i32;
                y = y2 as i32;
                xe = x1 as i32;
            }
            let mut point = Vec3::new(x as f32, y as f32, 1.0);
            self.set_pixel(&point, &line_color);

            while x < xe {
                x = x + 1;
                if px < 0 {
                    px = px + 2 * dy1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        y = y + 1;
                    } else {
                        y = y - 1;
                    }
                    px = px + 2 * (dy1 - dx1);
                }
                point = Vec3::new(x as f32, y as f32, 1.0);
                self.set_pixel(&point, &line_color);
            }
        } else {
            if dy >= 0 {
                x = x1 as i32;
                y = y1 as i32;
                ye = y2 as i32;
            } else {
                x = x2 as i32;
                y = y2 as i32;
                ye = y1 as i32;
            }

            let mut point = Vec3::new(x as f32, y as f32, 1.0);
            self.set_pixel(&point, &line_color);

            while y < ye {
                y = y + 1;

                if py <= 0 {
                    py += 2 * dx1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        x = x + 1;
                    } else {
                        x = x - 1;
                    }
                    py = py + 2 * (dx1 - dy1);
                }

                point = Vec3::new(x as f32, y as f32, 1.0);
                self.set_pixel(&point, &line_color);
            }
        }
    }

    fn rasterize_wireframe(&mut self, t: &Triangle) {
        self.draw_line(t.c(), t.a(), t.color[0]);
        self.draw_line(t.c(), t.b(), t.color[1]);
        self.draw_line(t.b(), t.a(), t.color[2]);
    }

    fn get_index(&self, x: u32, y: u32) -> u32 {
        return (self.height - y) * self.width + x;
    }

    fn get_next_id(&mut self) -> u32 {
        self.next_id += 1;
        return self.next_id;
    }

    pub fn data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                std::mem::transmute(self.frame_buf.as_ptr()),
                self.frame_buf.len() * 3,
            )
        }
    }
}

fn to_vec4(v3: Vec3, w: f32) -> Vec4 {
    return Vec4::new(v3.x, v3.y, v3.z, w);
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
