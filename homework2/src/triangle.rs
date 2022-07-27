use glam::{Vec2, Vec3, Vec4};

#[derive(Clone, Copy, Default, Debug)]
pub struct Rgb(u8, u8, u8);

impl From<&Vec3> for Rgb {
    fn from(i: &Vec3) -> Self {
        Self(i.x as u8, i.y as u8, i.z as u8)
    }
}

#[derive(Default)]
pub struct Triangle {
    // The original coordinates of the triangle,
    // v0, v1, v2 in counter clockwise order
    pub v: [Vec3; 3],

    // Per vertex values
    pub color: [Rgb; 3],       // color at each vertex
    pub tex_coords: [Vec2; 3], // texture u,v
    pub normal: [Vec3; 3],     // normal vector for each vertex
}

impl Triangle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn a(&self) -> Vec3 {
        return self.v[0];
    }

    pub fn b(&self) -> Vec3 {
        return self.v[1];
    }

    pub fn c(&self) -> Vec3 {
        return self.v[2];
    }

    // set i-th vertex coordinates
    pub fn set_vertex(&mut self, index: usize, v: Vec3) {
        self.v[index] = v;
    }

    // set i-th vertex normal vector
    pub fn set_normal(&mut self, index: usize, n: Vec3) {
        self.normal[index] = n;
    }

    // set i-th vertex color
    pub fn set_color(&mut self, index: usize, rgb: Rgb) {
        self.color[index] = rgb;
    }

    // Only one color per triangle.
    pub fn get_color(&self) -> Rgb {
        return self.color[0];
    }

    // set i-th vertex texture coordinates
    pub fn set_tex_coords(&mut self, index: usize, u: f32, v: f32) {
        self.tex_coords[index] = Vec2::new(u, v);
    }

    pub fn to_vec4(&self) -> [Vec4; 3] {
        return [
            Vec4::new(self.v[0].x, self.v[0].y, self.v[0].z, 1.0),
            Vec4::new(self.v[1].x, self.v[1].y, self.v[1].z, 1.0),
            Vec4::new(self.v[2].x, self.v[2].y, self.v[2].z, 1.0),
        ];
    }
}
