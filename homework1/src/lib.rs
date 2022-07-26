use std::f32::consts::PI;

use glam::{Mat4, Vec3, Vec4};

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
    let (sin, cos) = (rotation_angle * PI / 180.0).sin_cos();
    Mat4::from_cols(
        Vec4::new(cos, -sin, 0.0, 0.0),
        Vec4::new(sin, cos, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
    .transpose()
}

pub fn get_projection_matrix(eye_fov: f32, aspect_radio: f32, z_near: f32, z_far: f32) -> Mat4 {
    // eye_fov: deg
    // aspect_radio 1.0
    // z_near 0.1
    // z_far 50.0
    // near -> not moved
    // x_n/z_n = x/z
    // (x,y,z,1) -> (xn/-z,yn/-z,?,1) -> (xn, yn, ?, -z)
    // (?,?,n,1) -> (?,?,n,1)
    // (0,0,f,1) -> (0,0,?,f)
    let m1 = Mat4::from_cols(
        Vec4::new(z_near, 0.0, 0.0, 0.0),
        Vec4::new(0.0, z_near, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
        Vec4::new(0.0, 0.0, -1.0, 0.0),
    )
    .transpose();

    let delta_x = z_near * (eye_fov * PI / 2.0 / 180.0).tan();
    let delta_y = delta_x * aspect_radio;
    let delta_z = (z_near - z_far) / 2.0;
    let center_z = (z_near + delta_z) / 2.0;

    // let m2_move = Mat4::from_cols(
    //     Vec4::new(1.0, 0.0, 0.0, 0.0),
    //     Vec4::new(0.0, 1.0, 0.0, 0.0),
    //     Vec4::new(0.0, 0.0, 1.0, -(z_far + z_near) / 2.0),
    //     Vec4::new(0.0, 0.0, 0.0, 1.0),
    // )
    // .transpose();
    // let m2_scale = Mat4::from_cols(
    //     Vec4::new(1.0/delta_x, 0.0, 0.0, 0.0),
    //     Vec4::new(0.0, 1.0/delta_y, 0.0, 0.0),
    //     Vec4::new(0.0, 0.0, 1.0/delta_z,0.0),
    //     Vec4::new(0.0, 0.0, 0.0, 1.0),
    // )
    // .transpose();
    // let m2 = m2_scale * m2_move;

    let m2 = Mat4::from_cols(
        Vec4::new(1.0 / delta_x, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0 / delta_y, 0.0, 0.0),
        Vec4::new(0.0, 0.0, center_z / delta_z, -center_z),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
    .transpose();
    m2 * m1
}
