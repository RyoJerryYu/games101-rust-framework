use std::f32::consts::PI;

use glam::{Vec3, Mat3, Vec2};

pub fn your_code() -> Vec3 {
    let p = Vec3::new(2.0, 1.0, 1.0);
    let m1 = Mat3::from_angle(PI*45.0/180.0);
    let m2 = Mat3::from_translation(Vec2{x: 1.0, y: 2.0});
    m2 * m1 * p
}
