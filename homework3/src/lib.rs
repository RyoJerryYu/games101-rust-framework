pub mod rst;
pub mod shader;

use std::f32::consts::PI;

use glam::{Mat4, Vec3, Vec4};

/**
 * for every vertex on the model, should transform:
 * 1. apply model matrix to place the model to the right position in the world
 * 2. apply view matrix to reverse the translation of the eye position
 * 3. apply projection matrix to project the 3D world x(1..-1), y(1..-1), z(0..1) cube,
 *
 * after transform, map x, y to the screen space x(0..width), y(0..height)
 */

/**
 * get_view_matrix: reverse the translation of the eye position
 * after applying, the vertex will be in as if the eye is at the origin facing to -z axis
 * @eye_pos: the position of the eye
 * return: the view matrix
 */
pub fn get_view_matrix(eye_pos: Vec3) -> Mat4 {
    Mat4::from_cols(
        Vec4::new(1.0, 0.0, 0.0, -eye_pos.x),
        Vec4::new(0.0, 1.0, 0.0, -eye_pos.y),
        Vec4::new(0.0, 0.0, 1.0, -eye_pos.z),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
    .transpose()
}

/**
 * get_model_matrix: build the translation matrix
 * for placing the model to the position in the space.
 * @angle: the degree of rotation around the y axis, facing the negative y axis, counter-clockwise
 * @scale: the scale of the model
 * return: the model matrix
 */
pub fn get_model_matrix(angle: f32, scale: f32) -> Mat4 {
    // translate * scale * rotation
    let scale = Mat4::from_cols(
        Vec4::new(scale, 0.0, 0.0, 0.0),
        Vec4::new(0.0, scale, 0.0, 0.0),
        Vec4::new(0.0, 0.0, scale, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
    .transpose();

    let (sin, cos) = (angle * PI / 180.0).sin_cos();
    let rotate = Mat4::from_cols(
        Vec4::new(cos, 0.0, sin, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(-sin, 0.0, cos, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
    .transpose();
    scale * rotate
}

/**
 * get_projection_matrix: build the projection matrix
 * for projecting the 3D world x(1..-1), y(1..-1), z(-1..1) cube.
 *
 * this matrix do 2 things and is a multiplication of 2 matrices:
 * 1. map the eye coordinates pyramid frustum to the cube{-dx..dx, -dy..dy, znear..zfar}
 * 2. map the cube{-dx..dx, -dy..dy, znear..zfar} to the cube{-1..1, -1..1, -1..1}
 *
 * @eye_fov: the angle of the eye field of view, in degree
 * @aspect_ratio: the aspect ratio of the screen, height / width
 * @z_near: the near plane of the eye coordinates pyramid frustum
 * @z_far: the far plane of the eye coordinates pyramid frustum
 * return: the projection matrix
 */
pub fn get_projection_matrix(eye_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
    /*
     * m1: mapping the eye coordinates pyramid frustum to the cube{-dx..dx, -dy..dy, znear..zfar}
     * (x,y,z_near) -> (x,y,z_near)
     * (0,0,z_far) -> (0,0,z_far)
     *
     * after m1, every point (x, y, z) transformed to (x2, y2, z2),
     * which (x2, y2, z_near) is the projection of (x, y, z) on the near plane,
     * and that projection should provide the same x and y ratio as the original point.
     * so: (x2, y2) / z_near = (x, y) / z
     * (x,y,z) -> (x/z * z_near, y/z * z_near, z2) // which z2 we don't know yet
     */

    // 1. (x,y,z,1) -> (x*z_near, y*z_near, ?, z)
    // 2. (x,y,z_near,1) -> (x*z_near, y*z_near, z_near*z_near, z_near)
    // 3. (0,0,z_far,1) -> (0,0,z_far*z_far, z_far)
    // a*z_near + b = z_near * z_near
    // a*z_far + b = z_far * z_far
    let z_near = -z_near;
    let z_far = -z_far;
    let m1 = Mat4::from_cols(
        Vec4::new(z_near, 0.0, 0.0, 0.0),
        Vec4::new(0.0, z_near, 0.0, 0.0),
        Vec4::new(0.0, 0.0, z_near + z_far, -z_near * z_far),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
    )
    .transpose();

    /*
     * m2: mapping the cube{-dx..dx, -dy..dy, znear..zfar} to the cube{-1..1, -1..1, -1..1}
     * eye_fov = 2 * atan(dx / |z_near| )// but z_near is negative!!!
     * aspect_ratio = dy / dx
     */

    // make sure delta_x, delta_y, delta_z are positive
    // center_z is negative, but that's exactly what representing the center z position
    let delta_x = -z_near * (eye_fov * PI / 2.0 / 180.0).tan();
    let delta_y = delta_x * aspect_ratio;
    let delta_z = (z_near - z_far) / 2.0;
    let center_z = (z_near + delta_z) / 2.0;

    // m2 is a multiplication of 2 matrixs:
    // 1. first translate the center point (0,0,center_z) to (0,0,0)
    //    note that moving vector is (0,0,-center_z), that's wy the line3 should multiple negative
    // 2. then scalling Cube(2dx, 2dy, 2dz) to Cube(2, 2, 2)
    let m2 = Mat4::from_cols(
        Vec4::new(1.0 / delta_x, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0 / delta_y, 0.0, 0.0),
        Vec4::new(0.0, 0.0, -center_z / delta_z, -center_z / delta_z),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    )
    .transpose();
    m2 * m1
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
    let texture_color = match payload.texture {
        Some(texture) => {
            // TODO: Get the texture value at the texture coordinates of the current fragment
            texture
                .get_color(payload.tex_coords.x, payload.tex_coords.y)
                .into()
        }
        None => Vec3::new(0.0, 0.0, 0.0),
    };

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
        // TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular*
        // components are. Then, accumulate that result on the *result_color* object.
        todo!()
    }
    result_color
}

pub fn phong_fragment_shader(payload: &shader::FragmentShaderPayload) -> Vec3 {
    let ka = Vec3::ONE * 0.005;
    let kd = payload.color / 255.0;
    let ks = Vec3::ONE * 0.7937;

    let l1 = Light {
        position: Vec3::ONE * 20.0,
        intensity: Vec3::ONE * 500.,
    };
    let l2 = Light {
        position: Vec3::new(-20., 20., 0.),
        intensity: Vec3::ONE * 500.,
    };

    let lights = vec![l1, l2];
    let amb_light_intensity = Vec3::ONE * 10.;
    let eye_pos = Vec3::new(0.0, 0.0, 10.0);

    let p = 150.;

    let color = payload.color;
    let point = payload.view_pos;
    let normal = payload.normal.normalize();

    let mut result_color = Vec3::ZERO;
    for light in lights {
        // TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular*
        // components are. Then, accumulate that result on the *result_color* object.

        // ambient 环境光
        // diffuse 散射
        // specular 镜面反射

        let light_dir = light.position - point; // light_dir represent a vector from shading point to light position
        let eye_dir = eye_pos - point; // eye_dir represent a vector from shading point to eye position
        let reg_light_intensity = light.intensity / light_dir.dot(light_dir); // I/(r^2) , represent the energy arrived shading point

        // let kd = kd * 0.0;
        // let ks = ks * 0.0;
        let la = ka * amb_light_intensity;
        let ld = kd * reg_light_intensity * light_dir.normalize().dot(normal).max(0.0);
        let ls = ks * reg_light_intensity * (light_dir + eye_dir).normalize().dot(normal).max(0.0).powf(p);

        result_color += la + ld + ls;
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

    // TODO: Implement displacement mapping here
    // Let n = normal = (x, y, z)
    // Vector t = (x*y/sqrt(x*x+z*z),sqrt(x*x+z*z),z*y/sqrt(x*x+z*z))
    // Vector b = n cross product t
    // Matrix TBN = [t b n]
    // dU = kh * kn * (h(u+1/w,v)-h(u,v))
    // dV = kh * kn * (h(u,v+1/h)-h(u,v))
    // Vector ln = (-dU, -dV, 1)
    // Position p = p + kn * n * h(u,v)
    // Normal n = normalize(TBN * ln)

    let mut result_color = Vec3::ZERO;

    for light in lights {
        todo!()
        // TODO: For each light source in the code, calculate what the *ambient*, *diffuse*, and *specular*
        // components are. Then, accumulate that result on the *result_color* object.
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
    // TODO: Implement bump mapping here
    // Let n = normal = (x, y, z)
    // Vector t = (x*y/sqrt(x*x+z*z),sqrt(x*x+z*z),z*y/sqrt(x*x+z*z))
    // Vector b = n cross product t
    // Matrix TBN = [t b n]
    // dU = kh * kn * (h(u+1/w,v)-h(u,v))
    // dV = kh * kn * (h(u,v+1/h)-h(u,v))
    // Vector ln = (-dU, -dV, 1)
    // Normal n = normalize(TBN * ln)

    let mut result_color = Vec3::ZERO;
    result_color = normal;

    result_color * 255.
}

#[cfg(test)]
mod test {
    use glam::Vec2;

    use super::*;

    #[test]
    fn test_multiple() {
        // from cols should transpose, and the matrix matches each row
        fn scale(s: f32) -> Mat4 {
            Mat4::from_cols(
                Vec4::new(s, 0., 0., 0.),
                Vec4::new(0., s, 0., 0.),
                Vec4::new(0., 0., s, 0.),
                Vec4::new(0., 0., 0., s),
            )
            .transpose()
        }

        fn triangle() -> Mat4 {
            Mat4::from_cols(
                Vec4::new(1., 1., 1., 1.),
                Vec4::new(0., 1., 1., 1.),
                Vec4::new(0., 0., 1., 1.),
                Vec4::new(0., 0., 0., 1.),
            )
            .transpose()
        }

        struct TestCase {
            input: Vec4,
            mat: Mat4,
            expected: Vec4,
        }

        let cases = &[
            TestCase {
                input: Vec4::ZERO,
                mat: scale(2.),
                expected: Vec4::new(0., 0., 0., 0.),
            },
            TestCase {
                input: Vec4::X,
                mat: scale(2.),
                expected: Vec4::new(2., 0., 0., 0.),
            },
            TestCase {
                input: Vec4::Y,
                mat: scale(2.),
                expected: Vec4::new(0., 2., 0., 0.),
            },
            TestCase {
                input: Vec4::X,
                mat: triangle(),
                expected: Vec4::new(1., 0., 0., 0.),
            },
            TestCase {
                input: Vec4::Y,
                mat: triangle(),
                expected: Vec4::new(1., 1., 0., 0.),
            },
            TestCase {
                input: Vec4::Z,
                mat: triangle(),
                expected: Vec4::new(1., 1., 1., 0.),
            },
            TestCase {
                input: Vec4::W,
                mat: triangle(),
                expected: Vec4::new(1., 1., 1., 1.),
            },
            TestCase {
                input: Vec4::ONE,
                mat: triangle(),
                expected: Vec4::new(4., 3., 2., 1.),
            },
            TestCase {
                input: Vec4::new(1., 2., 3., 4.),
                mat: triangle(),
                expected: Vec4::new(10., 9., 7., 4.),
            },
        ];

        for case in cases {
            let result = case.mat * case.input;
            assert_eq!(result, case.expected);
        }
    }

    #[test]
    fn test_get_view_matrix() {
        let eye_pos = Vec3::new(0.0, 0.0, 10.0);
        let view_matrix = get_view_matrix(eye_pos);

        // every input point should move 10 units towards the negative z-axis

        let test_cases = &[
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 10.0),
            Vec3::new(0.0, 0.0, 20.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(1.0, 1.0, 10.0),
            Vec3::new(-1.0, -1.0, 10.0),
        ];

        for test_case in test_cases {
            let result = view_matrix.transform_point3(*test_case);
            assert_eq!(result, *test_case - Vec3::new(0.0, 0.0, 10.0));
        }
    }

    #[test]
    fn test_get_model_matrix_scale() {
        // scale should be multiplied to the x, y, and z components of the input

        struct TestCase {
            input: Vec3,
            scale: f32,
        }

        let cases = &[
            TestCase {
                input: Vec3::new(0.0, 0.0, 0.0),
                scale: 1.0,
            },
            TestCase {
                input: Vec3::new(0.0, 0.0, 0.0),
                scale: 2.0,
            },
            TestCase {
                input: Vec3::new(1.0, 0.0, 0.0),
                scale: 1.0,
            },
            TestCase {
                input: Vec3::new(0.0, 1.0, 0.0),
                scale: 2.0,
            },
            TestCase {
                input: Vec3::new(0.0, 0.0, 1.0),
                scale: 3.0,
            },
        ];

        for case in cases {
            let model_matrix = get_model_matrix(0.0, case.scale);
            let result = model_matrix.transform_point3(case.input);
            assert_eq!(result, case.input * case.scale);
        }
    }

    #[test]
    fn test_get_model_matrix_rotate_lib() {
        let cases = [0.0, 90.0, -90.0, 180.0, -180.0];

        for case in cases {
            let model_matrix = get_model_matrix(case, 1.0);
            let result = Mat4::from_rotation_y(case.to_radians());
            assert_eq!(model_matrix, result);
        }
    }

    #[test]
    fn test_get_model_matrix_rotate() {
        // rotate should rotate the input around the y-axis,
        // facing the negative y-axis, rotating counter-clockwise

        #[derive(Debug)]
        struct TestCase {
            input: Vec3,
            angle: f32, // in degrees
            expected: Vec3,
        }

        let cases = &[
            TestCase {
                input: Vec3::ZERO,
                angle: 0.0,
                expected: Vec3::ZERO,
            },
            TestCase {
                input: Vec3::X,
                angle: 0.0,
                expected: Vec3::X,
            },
            TestCase {
                input: Vec3::ONE,
                angle: 0.0,
                expected: Vec3::ONE,
            },
            TestCase {
                input: Vec3::X,
                angle: 90.0,
                expected: -Vec3::Z,
            },
            TestCase {
                input: Vec3::Z,
                angle: 90.0,
                expected: Vec3::X,
            },
            TestCase {
                input: Vec3::Y,
                angle: 90.0,
                expected: Vec3::Y,
            },
            TestCase {
                input: Vec3::new(1., 1., 1.),
                angle: 90.0,
                expected: Vec3::new(1., 1., -1.),
            },
        ];

        for case in cases {
            dbg!(case);
            let model_matrix = get_model_matrix(case.angle, 1.0);
            let result = model_matrix * case.input.extend(1.0);
            let result = result.truncate();
            assert!(result.abs_diff_eq(case.expected, f32::EPSILON));
        }
    }

    fn get_projection_matrix_m1(z_far: f32, z_near: f32) -> Mat4 {
        /*
         * (x,y,z) -> (x/z * z_near, y/z * z_near, z2) // which z2 we don't know yet
         * (x,y,z_near) -> (x,y,z_near)
         * (0,0,z_far) -> (0,0,z_far)
         */
        // 1. (x,y,z,1) -> (x*z_near, y*z_near, ?, z)
        // 2. (x,y,z_near,1) -> (x*z_near, y*z_near, z_near*z_near, z_near)
        // 3. (0,0,z_far,1) -> (0,0,z_far*z_far, z_far)
        // a*z_near + b = z_near * z_near
        // a*z_far + b = z_far * z_far
        let a = z_near + z_far;
        let b = -z_near * z_far;
        Mat4::from_cols(
            Vec4::new(z_near, 0.0, 0.0, 0.0),
            Vec4::new(0.0, z_near, 0.0, 0.0),
            Vec4::new(0.0, 0.0, a, b),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
        )
        .transpose()
    }

    #[test]
    fn test_projection_matrix_m1() {
        let z_near_far_list = [
            (0.1, 100.0),
            (1.0, 100.0),
            (1.0, 1000.0),
            (10.0, 100.0),
            (10.0, 1000.0),
        ];

        // CASE 1
        // for every point (x, y, z) transform to (x2, y2, z2)
        // (x, y, z) should have the same ratio as (x2, y2, z_near)
        let test_cases_1 = &[
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 1.0),
            Vec3::new(1.0, 0.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(1.0, 2.0, 1.0),
            Vec3::new(2.0, 3.0, 2.0),
            Vec3::new(3.0, 2.0, 1.0),
        ];

        for (z_near, z_far) in z_near_far_list {
            let projection_matrix = get_projection_matrix_m1(z_far, z_near);
            for test_case in test_cases_1 {
                let point = *test_case;
                let result = projection_matrix * point.extend(1.0);
                let result = result.truncate() / result.w;
                assert_eq!(result.x / z_near, point.x / point.z);
                assert_eq!(result.y / z_near, point.y / point.z);
            }
        }

        // CASE 2
        // every point on z_near should not change
        let test_cases_2 = &[
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(2.0, 3.0),
        ];
        for (z_near, z_far) in z_near_far_list {
            let projection_matrix = get_projection_matrix_m1(z_far, z_near);
            for test_case in test_cases_2 {
                let point = test_case.extend(z_near);
                let result = projection_matrix * point.extend(1.0);
                let result = result.truncate() / result.w;
                dbg!(point, result);
                assert!(result.abs_diff_eq(point, 0.0001));
            }
        }

        // CASE 3
        // the zero point on z_far should not change
        for (z_near, z_far) in z_near_far_list {
            let projection_matrix = get_projection_matrix_m1(z_far, z_near);
            let point = Vec2::ZERO.extend(z_far);
            let result = projection_matrix * point.extend(1.0);
            let result = result.truncate() / result.w;
            dbg!(point, result);
            assert!(result.abs_diff_eq(point, f32::EPSILON));
        }
    }
}
