#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use glam::Vec4;
    use homework1::{get_model_matrix, get_projection_matrix};

    #[test]
    fn your_test() {
        let p1 = Vec4::new(1.0, 0.0, 0.0, 1.0);
        let res = get_model_matrix(90.0) * p1;
        debug_assert!(res.abs_diff_eq(Vec4::new(0.0, 1.0, 0.0, 1.0), 0.00001));

        let p2 = Vec4::new(0.0, 1.0, 0.0, 1.0);
        let res = get_model_matrix(90.0) * p2;
        debug_assert!(res.abs_diff_eq(Vec4::new(-1.0, 0.0, 0.0, 1.0), 0.00001));

        let p3 = Vec4::new(0.0, 0.0, 1.0, 1.0);
        let res = get_model_matrix(90.0) * p3;
        debug_assert!(res.abs_diff_eq(Vec4::new(0.0, 0.0, 1.0, 1.0), 0.00001));
    }
}
