use std::f32::consts::PI;

use glam::{Mat4, Vec2, Vec3, Vec4};

pub mod triangle;

use crate::triangle::Triangle;

pub fn get_view_matrix(eye_pos: Vec3) -> Mat4 {
    Mat4::from_cols(
        Vec4::new(1.0, 0.0, 0.0, -eye_pos.x),
        Vec4::new(0.0, 1.0, 0.0, -eye_pos.y),
        Vec4::new(0.0, 0.0, 1.0, -eye_pos.z),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
    .transpose()
}

pub fn get_model_matrix(rotation_angle: f32) -> Mat4 {
    let model = Mat4::IDENTITY;
    model
}

pub fn get_projection_matrix(eye_fov: f32, aspect_radio: f32, z_near: f32, z_far: f32) -> Mat4 {
    // TODO: Copy-paste your implementation from the previous assignment.
    let projection = Mat4::IDENTITY;
    projection
}

pub fn inside_triangle(xc: f32, yc: f32, t: &Triangle) -> bool {
    // TODO : Implement this function to check if the point (x, y)
    // is inside the triangle represented by _v[0], _v[1], _v[2]
    true
}
