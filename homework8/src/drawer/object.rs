use glam::Vec2;
use utils::triangle::Rgb;

use super::types::{Object, XYBound};

pub struct Line {
    p1: Vec2,
    p2: Vec2,
    color: Rgb,

    // calculated
    half_width: f32,
    direction: Vec2,
    distence_2_threshold: f32,
}

impl Line {
    pub fn new(p1: Vec2, p2: Vec2, color: &Rgb) -> Self {
        let line_width = 2.0;
        let half_width = line_width / 2.0;
        Self {
            p1,
            p2,
            color: *color,
            half_width,
            direction: (p2 - p1).normalize(),
            distence_2_threshold: half_width * half_width,
        }
    }
}

impl Object for Line {
    fn get_bound(&self) -> XYBound<f32> {
        XYBound {
            min_x: self.p1.x.min(self.p2.x) - self.half_width,
            max_x: self.p1.x.max(self.p2.x) + self.half_width,
            min_y: self.p1.y.min(self.p2.y) - self.half_width,
            max_y: self.p1.y.max(self.p2.y) + self.half_width,
        }
    }

    fn is_in_bound(&self, p: Vec2) -> bool {
        if self.direction.dot(p - self.p1) < 0.0 {
            return false;
        }
        if -self.direction.dot(p - self.p2) < 0.0 {
            return false;
        }

        // let H = perpendicular for P on line , O = p1
        // => OH = P perspect on line = OP dot direction * dirction
        // distance = length of OP - OH
        let op = p - self.p1;
        if (op - op.dot(self.direction) * self.direction).length_squared() > self.distence_2_threshold {
            return false;
        }
        return true;
    }

    fn get_color(&self) -> &utils::triangle::Rgb {
        &self.color
    }
}
