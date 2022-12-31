pub mod rst;
pub mod shader;

use std::f32::consts::PI;

use glam::{Mat4, Vec3};

pub fn get_view_matrix(eye_pos: Vec3) -> Mat4 {
    let mut view = Mat4::IDENTITY;

    let translate = Mat4::from_cols_array_2d(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-eye_pos.x, -eye_pos.y, -eye_pos.z, 1.0],
    ])
    .transpose();

    view * translate
}

pub fn get_model_matrix(angle: f32) -> Mat4 {
    let angle = angle * PI / 180.0;

    let rotation = Mat4::from_cols_array_2d(&[
        [angle.cos(), 0.0, angle.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-angle.sin(), 0.0, angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
    .transpose();

    let scale = Mat4::from_cols_array_2d(&[
        [2.5, 0.0, 0.0, 0.0],
        [0.0, 2.5, 0.0, 0.0],
        [0.0, 0.0, 2.5, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
    .transpose();

    let translate = Mat4::from_cols_array_2d(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
    .transpose();

    translate * scale * rotation
}

pub fn get_projection_matrix(eye_fov: f32, aspect_ratio: f32, znear: f32, zfar: f32) -> Mat4 {
    todo!()
}

pub fn vertex_shader(payload: &shader::VertexShaderPayload) -> Vec3 {
    return payload.position;
}

pub fn normal_fragment_shader(payload: &shader::FragmentShaderPayload) -> Vec3 {
    let return_color = payload.normal.normalize() + Vec3::new(1.0, 1.0, 1.0) / 2.0;
    let result = Vec3::from_array([
        return_color.x * 255.,
        return_color.y * 255.,
        return_color.z * 255.,
    ]);

    result
}

fn reflect(vec: Vec3, axis: Vec3) -> Vec3 {
    let costheta = vec.dot(axis);
    return (2. * costheta * axis - vec).normalize();
}

struct Light {
    position: Vec3,
    intensity: Vec3,
}

pub fn texture_fragment_shader(payload: &shader::FragmentShaderPayload) -> Vec3 {
    let mut return_color = Vec3::new(0.0, 0.0, 0.0);
    if payload.texture.is_some() {
        todo!()
    }
    let mut texture_color = Vec3::new(return_color.x, return_color.y, return_color.z);

    let ka = Vec3::new(0.005, 0.005, 0.005);
    let kd = texture_color / 255.0;
    let ks = Vec3::new(0.7937, 0.7937, 0.7937);

    let l1 = Light {
        position: Vec3::ONE * 20.0,
        intensity: Vec3::ONE * 500.,
    };
    let l2 = Light {
        position: Vec3::new(-20., -20., 0.),
        intensity: Vec3::ONE * 500.,
    };

    let lights = vec![l1, l2];
    let amb_light_intensity = Vec3::ONE * 10.;
    let eye_pos = Vec3::new(0.0, 0.0, 10.0);

    let p = 150.;

    let color = texture_color;
    let point = payload.view_pos;
    let normal = payload.normal;
    let mut result_color = Vec3::ZERO;

    for light in lights {
        todo!()
    }
    result_color
}

pub fn phong_fragment_shader(payload: &shader::FragmentShaderPayload) -> Vec3 {
    let ka = Vec3::ONE * 0.005;
    let kd = payload.color;
    let ks = Vec3::ONE * 0.7937;

    let l1 = Light {
        position: Vec3::ONE * 20.0,
        intensity: Vec3::ONE * 500.,
    };
    let l2 = Light {
        position: Vec3::new(-20., -20., 0.),
        intensity: Vec3::ONE * 500.,
    };

    let lights = vec![l1, l2];
    let amb_light_intensity = Vec3::ONE * 10.;
    let eye_pos = Vec3::new(0.0, 0.0, 10.0);

    let p = 150.;

    let color = payload.color;
    let point = payload.view_pos;
    let normal = payload.normal;

    let mut result_color = Vec3::ZERO;
    for light in lights {
        todo!()
    }

    result_color * 255.
}


pub fn displacement_fragment_shader(payload: &shader::FragmentShaderPayload) -> Vec3 {
    let ka = Vec3::ONE * 0.005;
    let kd = payload.color / 255.0;
    let ks = Vec3::ONE * 0.7937;

    let l1 = Light {
        position: Vec3::ONE * 20.0,
        intensity: Vec3::ONE * 500.,
    };
    let l2 = Light {
        position: Vec3::new(-20., -20., 0.),
        intensity: Vec3::ONE * 500.,
    };

    let lights = vec![l1, l2];
    let amb_light_intensity = Vec3::ONE * 10.;
    let eye_pos = Vec3::new(0.0, 0.0, 10.0);

    let p = 150.;

    let color = payload.color;
    let point = payload.view_pos;
    let normal = payload.normal;

    let (kh, kn) = (0.2, 0.1);
    let mut result_color = Vec3::ZERO;

    for light in lights {
        todo!()
    }
    result_color * 255.
}

pub fn bump_fragment_shader(payload: &shader::FragmentShaderPayload) -> Vec3 {
    let ka = Vec3::ONE * 0.005;
    let kd = payload.color / 255.0;
    let ks = Vec3::ONE * 0.7937;

    let l1 = Light {
        position: Vec3::ONE * 20.0,
        intensity: Vec3::ONE * 500.,
    };
    let l2 = Light {
        position: Vec3::new(-20., -20., 0.),
        intensity: Vec3::ONE * 500.,
    };

    let lights = vec![l1, l2];
    let amb_light_intensity = Vec3::ONE * 10.;
    let eye_pos = Vec3::new(0.0, 0.0, 10.0);

    let p = 150.;

    let color = payload.color;
    let point = payload.view_pos;
    let normal = payload.normal;

    let (kh, kn) = (0.2, 0.1);
    todo!();

    let mut result_color = Vec3::ZERO;
    result_color = normal;

    result_color * 255.
}
