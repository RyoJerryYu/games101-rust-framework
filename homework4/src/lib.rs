use glam::Vec2;

pub mod rst;

pub fn naive_bezier(r: &mut rst::Rasterizer, control_points: &Vec<Vec2>) {
    let range_length = 1000;
    for t in 0..range_length {
        let t = t as f32 / range_length as f32;
        let sub_t = 1.0 - t;
        let point = sub_t.powi(3) * control_points[0]
            + 3.0 * t * sub_t.powi(2) * control_points[1]
            + 3.0 * t.powi(2) * sub_t * control_points[2]
            + t.powi(3) * control_points[3];

        r.pixel_add_rgb(&point, &utils::triangle::Rgb::RED);
    }
}

fn recursive_bezier(control_points: &Vec<Vec2>, t: f32) -> Vec2 {
    // TODO: Implement de Casteljau's algorithm
    if control_points.len() <= 1 {
        return control_points[0];
    }
    let mut points = vec![];
    for i in 0..control_points.len() - 1 {
        points.push((1.0 - t) * control_points[i] + t * control_points[i + 1]);
    }
    recursive_bezier(&points, t)
}

pub fn bezier(r: &mut rst::Rasterizer, control_points: &Vec<Vec2>) {
    // TODO: Iterate through all t = 0 to t = 1 with small steps, and call de Casteljau's
    // recursive Bezier algorithm.
    let range_length = 1000;
    for t in 0..range_length {
        let t = t as f32 / range_length as f32;
        let point = recursive_bezier(control_points, t);
        r.pixel_add_rgb(&point, &utils::triangle::Rgb::GREEN);
    }
}
