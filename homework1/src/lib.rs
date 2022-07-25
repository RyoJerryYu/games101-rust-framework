use glam::{Mat4, Vec3, Vec4};

pub fn get_view_matrix(eye_pos: Vec3) -> Mat4 {
    let mut view = Mat4::IDENTITY;
    let translate = Mat4::from_cols(
        Vec4::new(1.0, 0.0, 0.0, -eye_pos.x),
        Vec4::new(0.0, 1.0, 0.0, -eye_pos.y),
        Vec4::new(0.0, 0.0, 1.0, -eye_pos.z),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    );
    view = translate * view;
    view
}

pub fn get_model_matrix(rotation_angle: f32) -> Mat4 {
    let model = Mat4::IDENTITY;
    model
}

pub fn get_projection_matrix(eye_fov: f32, aspect_radio: f32, z_near: f32, z_far: f32) -> Mat4 {
    let projection = Mat4::IDENTITY;
    projection
}
