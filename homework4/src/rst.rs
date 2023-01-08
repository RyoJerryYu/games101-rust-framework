use glam::{Vec2, Vec3};
pub use utils::rasterizer::{Buffers, IndBufId, PosBufId, Primitive, Rasterizable};

pub struct Rasterizer {
    frame_buf: Vec<utils::triangle::Rgb>,
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
            frame_buf: Vec::new(),
            width,
            height,
        };

        res.frame_buf
            .resize((width * height) as usize, utils::triangle::Rgb::default());

        res
    }

    // origin point on top left of the screen,
    // x axis face to right and y axis face to down.
    fn set_pixel(&mut self, point: &Vec2, color: &utils::triangle::Rgb) {
        let (x_range, y_range) = (0..self.width, 0..self.height);
        let (x, y) = (point.x as u32, point.y as u32);
        if !(x_range.contains(&x) && y_range.contains(&y)) {
            return;
        }

        let ind = y * self.width + x;
        self.frame_buf[ind as usize] = *color;
    }

    pub fn clear(&mut self, buffers: Buffers) {
        if buffers.contains(Buffers::COLOR) {
            self.frame_buf.fill(utils::triangle::Rgb(0, 0, 0));
        }
    }

    pub fn draw_circle(&mut self, center: Vec2, radius: f32) {
        let max_x = (center.x + radius) as u32 + 1;
        let min_x = (center.x - radius) as u32;
        let max_y = (center.y + radius) as u32 + 1;
        let min_y = (center.y - radius) as u32;

        let max_x = max_x.clamp(0, self.width);
        let min_x = min_x.clamp(0, self.width);
        let max_y = max_y.clamp(0, self.height);
        let min_y = min_y.clamp(0, self.height);

        for x in min_x..max_x {
            for y in min_y..max_y {
                let (xc, yc) = (x as f32 + 0.5, y as f32 + 0.5);
                let dist_pow2 =
                    (xc - center.x) * (xc - center.x) + (yc - center.y) * (yc - center.y);
                if dist_pow2 > radius * radius {
                    continue;
                }

                self.set_pixel(&Vec2::new(xc, yc), &utils::triangle::Rgb(255, 255, 255));
            }
        }
    }
}
