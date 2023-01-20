use glam::Vec2;
use utils::{rasterizer::Rasterizable, triangle::Rgb};

#[derive(Clone, Copy, Debug)]
pub struct SimpleDrawerConfig {
    pub background: Rgb, // 0,0,0
    pub point_size: f32, // 6.0
    pub line_width: f32, // 1.0
}

impl SimpleDrawerConfig {
    pub fn default() -> Self {
        Self {
            background: Rgb(0, 0, 0),
            point_size: 6.0,
            line_width: 2.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct XYBound<T> {
    pub min_x: T,
    pub max_x: T,
    pub min_y: T,
    pub max_y: T,
}

// the coordinate in simple drawer is the pixel coordinate
pub struct SimpleDrawer {
    width: usize,
    height: usize,
    background: Rgb, // 0,0,0
    point_size: f32, // 3.0
    line_width: f32, // 3.0

    frame_buf: Vec<Rgb>,
}

impl Rasterizable for SimpleDrawer {
    fn data(&self) -> &Vec<utils::rgb::Rgb> {
        &self.frame_buf
    }

    fn size(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }
}

impl<'a> SimpleDrawer {
    pub fn new(width: usize, height: usize, conf: SimpleDrawerConfig) -> Self {
        let mut res = Self {
            width,
            height,
            background: conf.background,
            point_size: conf.point_size,
            line_width: conf.line_width,
            frame_buf: vec![],
        };

        res.frame_buf.resize(width * height, res.background);

        res
    }

    // return the buf index of point, if not in screen, return null
    // origin point on top left of the screen,
    // x axis face to right and y axis face to down.
    #[inline]
    fn buf_ind_at(&self, x: usize, y: usize) -> Option<usize> {
        let (x_range, y_range) = (0..self.width, 0..self.height);
        if !(x_range.contains(&x) && y_range.contains(&y)) {
            return None;
        }

        Some((y * self.width + x) as usize)
    }

    #[inline]
    fn set_pixel(&mut self, x: usize, y: usize, color: &Rgb) {
        match self.buf_ind_at(x, y) {
            None => (),
            Some(ind) => self.frame_buf[ind] = *color,
        }
    }

    // get color of pixel at point
    #[inline]
    fn get_pixel(&self, x: usize, y: usize) -> Option<Rgb> {
        match self.buf_ind_at(x, y) {
            None => None,
            Some(ind) => Some(self.frame_buf[ind]),
        }
    }

    #[inline]
    fn get_bound_for(&self, bound: XYBound<f32>) -> XYBound<usize> {
        XYBound {
            min_x: (bound.min_x as usize).clamp(0, self.width),
            max_x: (bound.max_x as usize).clamp(0, self.width) + 1,
            min_y: (bound.min_y as usize).clamp(0, self.height),
            max_y: (bound.max_y as usize).clamp(0, self.height) + 1,
        }
    }

    #[inline]
    fn get_center_for(&self, x: usize, y: usize) -> (f32, f32) {
        (x as f32 + 0.5, y as f32 + 0.5)
    }

    fn foreach_bound_pixel<F>(&mut self, coord_bound: XYBound<f32>, f: F)
    where
        F: Fn(Vec2) -> Option<&'a Rgb>,
    {
        let pixel_bound = self.get_bound_for(coord_bound);

        for x in pixel_bound.min_x..pixel_bound.max_x {
            for y in pixel_bound.min_y..pixel_bound.max_y {
                let (xc, yc) = self.get_center_for(x, y);
                let p = Vec2::new(xc, yc);
                if let Some(color) = f(p) {
                    self.set_pixel(x, y, &color);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.frame_buf.fill(self.background)
    }

    pub fn draw_circle(&mut self, center: Vec2, radius: f32, color: &Rgb) {
        let bound = XYBound {
            min_x: center.x - radius,
            max_x: center.x + radius,
            min_y: center.y - radius,
            max_y: center.y + radius,
        };

        self.foreach_bound_pixel(bound, |p| -> Option<&Rgb> {
            let dist_pow2 = (p - center).length_squared();
            if dist_pow2 > radius * radius {
                return None;
            }
            Some(color)
        });
    }

    pub fn draw_point(&mut self, p: Vec2, color: &Rgb) {
        self.draw_circle(p, self.point_size / 2.0, color)
    }

    pub fn draw_line(&mut self, p1: Vec2, p2: Vec2, color: &Rgb) {
        let half_width = self.line_width / 2.0;
        let l = p2 - p1;
        let direction = l.normalize();
        // threshold of d^2
        let distence_2_threshold = half_width * half_width;

        let bound = XYBound {
            min_x: p1.x.min(p2.x) - half_width,
            max_x: p1.x.max(p2.x) + half_width,
            min_y: p1.y.min(p2.y) - half_width,
            max_y: p1.y.max(p2.y) + half_width,
        };

        self.foreach_bound_pixel(bound, |p| -> Option<&Rgb> {
            if l.dot(p - p1) < 0.0 {
                return None;
            }
            if -l.dot(p - p2) < 0.0 {
                return None;
            }

            // let H = perpendicular for P on line , O = p1
            // => OH = P perspect on line = OP dot direction * dirction
            // distance = length of OP - OH
            let op = p - p1;
            if (op - op.dot(direction) * direction).length_squared() > distence_2_threshold {
                return None;
            }

            Some(color)
        });
    }
}

#[cfg(test)]
mod test {
    use utils::graphic::save_image;

    use super::*;
    #[test]
    fn test_drawer() {
        let mut drawer = SimpleDrawer::new(700, 700, SimpleDrawerConfig::default());
        let red = Rgb::new(255, 0, 0);
        let magenta = Rgb::new(255, 0, 255);
        let yellowgreen = Rgb::new(127, 255, 0);

        let p1 = Vec2::new(100.0, 100.0);
        let p2 = Vec2::new(100.0, 200.0);
        let p3 = Vec2::new(100.0, 300.0);
        drawer.draw_point(p1, &red);
        drawer.draw_line(p1, p2, &red);
        drawer.draw_point(p2, &red);
        drawer.draw_line(p2, p3, &red);
        drawer.draw_point(p3, &red);

        drawer.draw_circle(Vec2::new(300.0, 300.0), 100.0, &red);

        let p1 = Vec2::new(400.0, 400.0);
        let p2 = Vec2::new(500.0, 500.0);
        drawer.draw_point(p1, &red);
        drawer.draw_line(p1, p2, &red);
        drawer.draw_point(p2, &red);

        drawer.draw_line(Vec2::new(400.0, 500.0), Vec2::new(500.0, 400.0), &magenta);
        drawer.draw_line(
            Vec2::new(400.0, 600.0),
            Vec2::new(500.0, 300.0),
            &yellowgreen,
        );
        save_image(&drawer, "output.png").unwrap();
    }
}
